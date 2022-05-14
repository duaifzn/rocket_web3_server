use crate::contract_interface::proof_of_existence_interface;
use crate::database::Mongo;
use crate::dto::request_dto::{
    AddIssuerDto, DelIssuerDto, DeployContractDto, GetHashDto, HashDto, IsIssuerDto,
    NotarizeHashDto, RawDto, RevokeHashDto, TransferOwnershipDto,
};
use crate::dto::response_dto::{
    ApiResponse, BoolDto, ContractAddressDto, HashValueDto, SendHashDto, Sha256HashDto,
    TxAddressDto,
};
use crate::middleware::auth_guard::Token;
use crate::service::contract_service::insert_one_contract;
use crate::util::error_handle::{
    error_handle_of_reqwest, error_handle_of_string, error_handle_of_web3,
};
use crate::util::eth_node::EthNode;
use crate::util::vault::Vault;
use hex::FromHex;
use rocket::serde::json::Json;
use rocket::State;
use rocket_okapi::openapi;
use sha2::{Digest, Sha256};
use uuid::Uuid;
use web3::ethabi::FixedBytes;
use web3::types::{Bytes, H160, U256};

#[openapi]
#[get("/hash", data = "<body>")]
pub async fn sha256_hash(body: Json<RawDto>) -> Json<ApiResponse<Sha256HashDto>> {
    let mut hasher = Sha256::new();
    hasher.update(body.raw_data.as_bytes());
    let done = hasher.finalize();
    let hash_data = format!("{:X}", done);
    Json(ApiResponse {
        success: true,
        code: 200,
        json: Some(Sha256HashDto {
            hash_data: hash_data,
        }),
        message: None,
    })
}

#[openapi]
#[post("/hash", data = "<body>")]
pub async fn send_hash(
    eth_node: &State<EthNode>,
    body: Json<HashDto>,
) -> Json<ApiResponse<SendHashDto>> {
    let contract = eth_node.connect_contract_of_proof_of_existence(&body.address.to_owned());
    let contract_interface = proof_of_existence_interface::ProofOfExistence { contract: contract };

    let new_uuid = Uuid::new_v4().to_string();
    let mut hasher = Sha256::new();
    hasher.update(new_uuid.as_bytes());
    let done = hasher.finalize();
    let new_uuid_sha256 = format!("{:X}", done);

    let response = contract_interface
        .notarize_hash(
            &body.private_key.to_owned(),
            &new_uuid_sha256,
            &body.hash_data.to_owned(),
        )
        .await;
    match response {
        Ok(result) => Json(ApiResponse {
            success: true,
            code: 200,
            json: Some(SendHashDto {
                tx_hash: format!("{:#x}", result),
            }),
            message: None,
        }),
        Err(err) => Json(ApiResponse {
            success: false,
            code: 400,
            json: None,
            message: Some(err.to_string()),
        }),
    }
}

#[openapi]
#[get("/contract/isIssuer", data = "<body>")]
pub async fn is_issuer(
    vault: &State<Vault>,
    eth_node: &State<EthNode>,
    body: Json<IsIssuerDto>,
) -> Result<Json<ApiResponse<BoolDto>>, Json<ApiResponse<String>>> {
    let contract = eth_node.connect_contract_of_proof_of_existence(&body.address);
    let res = vault
        .get_one_account(&body.account_name)
        .await
        .map_err(error_handle_of_reqwest)?;

    let contract_address = EthNode::hex_str_to_bytes20(&body.address.replace("0x", ""))
        .map_err(error_handle_of_web3)?;
    let account_address = EthNode::hex_str_to_bytes20(&res.data.address.replace("0x", ""))
        .map_err(error_handle_of_web3)?;
    let target_address = EthNode::hex_str_to_bytes20(&body.target_address.replace("0x", ""))
        .map_err(error_handle_of_web3)?;

    let data = contract
        .abi()
        .function("isIssuer")
        .unwrap()
        .encode_input(&[web3::ethabi::Token::Address(H160::from(target_address))])
        .unwrap();

    let call_req = web3::types::CallRequest {
        from: Some(web3::types::Address::from(account_address)),
        to: Some(web3::types::Address::from(contract_address)),
        gas: None,
        gas_price: None,
        value: None,
        data: Some(Bytes(data)),
        transaction_type: None,
        access_list: None,
        max_fee_per_gas: None,
        max_priority_fee_per_gas: None,
    };
    let call_res = eth_node
        .web3
        .eth()
        .call(call_req, None)
        .await
        .map_err(error_handle_of_web3)?;
    println!("{:?}", call_res);
    let result_temp = contract
        .abi()
        .function("isIssuer")
        .unwrap()
        .decode_output(&call_res.0)
        .unwrap();
    let result = <bool as web3::contract::tokens::Detokenize>::from_tokens(result_temp).unwrap();
    Ok(Json(ApiResponse {
        success: true,
        code: 200,
        json: Some(BoolDto { result: result }),
        message: None,
    }))
}

#[openapi]
#[post("/contract/notarizeHash", data = "<body>")]
pub async fn notarize_hash(
    db: &State<Mongo>,
    vault: &State<Vault>,
    eth_node: &State<EthNode>,
    body: Json<NotarizeHashDto>,
) -> Result<Json<ApiResponse<TxAddressDto>>, Json<ApiResponse<String>>> {
    let contract = eth_node.connect_contract_of_proof_of_existence(&body.address);
    let res = vault
        .get_one_account(&body.account_name)
        .await
        .map_err(error_handle_of_reqwest)?;

    let key_sha256 = EthNode::sha256_hash(&body.key);
    let key = EthNode::hex_str_to_bytes32(&key_sha256).unwrap();
    let value_sha256 = EthNode::sha256_hash(&body.value);
    let value = EthNode::hex_str_to_bytes32(&value_sha256).unwrap();

    let data = contract
        .abi()
        .function("notarizeHash")
        .unwrap()
        .encode_input(&[
            web3::ethabi::Token::FixedBytes(key.to_vec()),
            web3::ethabi::Token::FixedBytes(value.to_vec()),
        ])
        .unwrap();

    let count = eth_node
        .get_transaction_count(&res.data.address.replace("0x", ""))
        .await
        .map_err(error_handle_of_web3)?;
    let raw_tx = vault
        .create_one_raw_transaction(&body.address, count.low_u64(), data)
        .await;
    let signed_tx = vault
        .sign_one_transaction(&body.account_name, raw_tx)
        .await
        .map_err(error_handle_of_reqwest)?;
    let tx_address = eth_node
        .send_raw_transaction(Bytes::from(
            hex::decode(signed_tx.data.signed_transaction).unwrap(),
        ))
        .await
        .map_err(error_handle_of_web3)?;

    Ok(Json(ApiResponse {
        success: true,
        code: 200,
        json: Some(TxAddressDto {
            tx_address: format!("{:?}", tx_address),
        }),
        message: None,
    }))
}

#[openapi]
#[get("/contract/getHash", data = "<body>")]
pub async fn get_hash(
    vault: &State<Vault>,
    eth_node: &State<EthNode>,
    body: Json<GetHashDto>,
) -> Result<Json<ApiResponse<HashValueDto>>, Json<ApiResponse<String>>> {
    let contract = eth_node.connect_contract_of_proof_of_existence(&body.address);
    let res = vault
        .get_one_account(&body.account_name)
        .await
        .map_err(error_handle_of_reqwest)?;

    let contract_address = EthNode::hex_str_to_bytes20(&body.address.replace("0x", ""))
        .map_err(error_handle_of_web3)?;
    let account_address = EthNode::hex_str_to_bytes20(&res.data.address.replace("0x", ""))
        .map_err(error_handle_of_web3)?;
    let key_sha256 = EthNode::sha256_hash(&body.key);
    let key = EthNode::hex_str_to_bytes32(&key_sha256).map_err(error_handle_of_web3)?;

    let data = contract
        .abi()
        .function("getHash")
        .unwrap()
        .encode_input(&[web3::ethabi::Token::FixedBytes(key.to_vec())])
        .unwrap();

    let call_req = web3::types::CallRequest {
        from: Some(web3::types::Address::from(account_address)),
        to: Some(web3::types::Address::from(contract_address)),
        gas: None,
        gas_price: None,
        value: None,
        data: Some(Bytes(data)),
        transaction_type: None,
        access_list: None,
        max_fee_per_gas: None,
        max_priority_fee_per_gas: None,
    };
    let call_res = eth_node
        .web3
        .eth()
        .call(call_req, None)
        .await
        .map_err(error_handle_of_web3)?;

    let result_temp = contract
        .abi()
        .function("getHash")
        .unwrap()
        .decode_output(&call_res.0)
        .unwrap();
    let result =
        <FixedBytes as web3::contract::tokens::Detokenize>::from_tokens(result_temp).unwrap();
    Ok(Json(ApiResponse {
        success: true,
        code: 200,
        json: Some(HashValueDto {
            hash_value: hex::encode(result),
        }),
        message: None,
    }))
}

#[openapi]
#[patch("/contract/revokeHash", data = "<body>")]
pub async fn revoke_hash(
    db: &State<Mongo>,
    vault: &State<Vault>,
    eth_node: &State<EthNode>,
    body: Json<RevokeHashDto>,
) -> Result<Json<ApiResponse<TxAddressDto>>, Json<ApiResponse<String>>> {
    let contract = eth_node.connect_contract_of_proof_of_existence(&body.address);
    let res = vault
        .get_one_account(&body.account_name)
        .await
        .map_err(error_handle_of_reqwest)?;

    let key_sha256 = EthNode::sha256_hash(&body.key);
    let key = EthNode::hex_str_to_bytes32(&key_sha256).map_err(error_handle_of_web3)?;
    let data = contract
        .abi()
        .function("revokeHash")
        .unwrap()
        .encode_input(&[web3::ethabi::Token::FixedBytes(key.to_vec())])
        .unwrap();

    let count = eth_node
        .get_transaction_count(&res.data.address.replace("0x", ""))
        .await
        .map_err(error_handle_of_web3)?;
    let raw_tx = vault
        .create_one_raw_transaction(&body.address, count.low_u64(), data)
        .await;
    let signed_tx = vault
        .sign_one_transaction(&body.account_name, raw_tx)
        .await
        .map_err(error_handle_of_reqwest)?;
    let tx_address = eth_node
        .send_raw_transaction(Bytes::from(
            hex::decode(signed_tx.data.signed_transaction).unwrap(),
        ))
        .await
        .map_err(error_handle_of_web3)?;

    Ok(Json(ApiResponse {
        success: true,
        code: 200,
        json: Some(TxAddressDto {
            tx_address: tx_address.to_string(),
        }),
        message: None,
    }))
}

#[openapi]
#[post("/contract/addIssuer", data = "<body>")]
pub async fn add_issuer(
    db: &State<Mongo>,
    vault: &State<Vault>,
    eth_node: &State<EthNode>,
    body: Json<AddIssuerDto>,
) -> Result<Json<ApiResponse<TxAddressDto>>, Json<ApiResponse<String>>> {
    let contract = eth_node.connect_contract_of_proof_of_existence(&body.address);
    let res = vault
        .get_one_account(&body.account_name)
        .await
        .map_err(error_handle_of_reqwest)?;

    let target_address = EthNode::hex_str_to_bytes20(&body.target_address.replace("0x", ""))
        .map_err(error_handle_of_web3)?;
    let data = contract
        .abi()
        .function("addIssuer")
        .unwrap()
        .encode_input(&[web3::ethabi::Token::Address(H160::from(target_address))])
        .unwrap();
    let count = eth_node
        .get_transaction_count(&res.data.address.replace("0x", ""))
        .await
        .map_err(error_handle_of_web3)?;
    let raw_tx = vault
        .create_one_raw_transaction(&body.address, count.low_u64(), data)
        .await;
    let signed_tx = vault
        .sign_one_transaction(&body.account_name, raw_tx)
        .await
        .map_err(error_handle_of_reqwest)?;
    let tx_address = eth_node
        .send_raw_transaction(Bytes::from(
            hex::decode(signed_tx.data.signed_transaction).unwrap(),
        ))
        .await
        .map_err(error_handle_of_web3)?;

    Ok(Json(ApiResponse {
        success: true,
        code: 200,
        json: Some(TxAddressDto {
            tx_address: tx_address.to_string(),
        }),
        message: None,
    }))
}

#[openapi]
#[delete("/contract/delIssuer", data = "<body>")]
pub async fn del_issuer(
    db: &State<Mongo>,
    vault: &State<Vault>,
    eth_node: &State<EthNode>,
    body: Json<DelIssuerDto>,
) -> Result<Json<ApiResponse<TxAddressDto>>, Json<ApiResponse<String>>> {
    let contract = eth_node.connect_contract_of_proof_of_existence(&body.address);
    let res = vault
        .get_one_account(&body.account_name)
        .await
        .map_err(error_handle_of_reqwest)?;
    let target_address = EthNode::hex_str_to_bytes32(&body.target_address.replace("0x", ""))
        .map_err(error_handle_of_web3)?;
    let data = contract
        .abi()
        .function("delIssuer")
        .unwrap()
        .encode_input(&[web3::ethabi::Token::FixedBytes(target_address.to_vec())])
        .unwrap();
    let count = eth_node
        .get_transaction_count(&res.data.address.replace("0x", ""))
        .await
        .map_err(error_handle_of_web3)?;
    let raw_tx = vault
        .create_one_raw_transaction(&body.address, count.low_u64(), data)
        .await;
    let signed_tx = vault
        .sign_one_transaction(&body.account_name, raw_tx)
        .await
        .map_err(error_handle_of_reqwest)?;
    let tx_address = eth_node
        .send_raw_transaction(Bytes::from(
            hex::decode(signed_tx.data.signed_transaction).unwrap(),
        ))
        .await
        .map_err(error_handle_of_web3)?;

    Ok(Json(ApiResponse {
        success: true,
        code: 200,
        json: Some(TxAddressDto {
            tx_address: tx_address.to_string(),
        }),
        message: None,
    }))
}

#[openapi]
#[patch("/contract/transferOwnership", data = "<body>")]
pub async fn transfer_ownership(
    db: &State<Mongo>,
    vault: &State<Vault>,
    eth_node: &State<EthNode>,
    body: Json<TransferOwnershipDto>,
) -> Result<Json<ApiResponse<TxAddressDto>>, Json<ApiResponse<String>>> {
    let contract = eth_node.connect_contract_of_proof_of_existence(&body.address);
    let res = vault
        .get_one_account(&body.account_name)
        .await
        .map_err(error_handle_of_reqwest)?;
    let target_address = EthNode::hex_str_to_bytes32(&body.target_address.replace("0x", ""))
        .map_err(error_handle_of_web3)?;
    let data = contract
        .abi()
        .function("delIssuer")
        .unwrap()
        .encode_input(&[web3::ethabi::Token::FixedBytes(target_address.to_vec())])
        .unwrap();
    let count = eth_node
        .get_transaction_count(&res.data.address.replace("0x", ""))
        .await
        .map_err(error_handle_of_web3)?;
    let raw_tx = vault
        .create_one_raw_transaction(&body.address, count.low_u64(), data)
        .await;
    let signed_tx = vault
        .sign_one_transaction(&body.account_name, raw_tx)
        .await
        .map_err(error_handle_of_reqwest)?;
    let tx_address = eth_node
        .send_raw_transaction(Bytes::from(
            hex::decode(signed_tx.data.signed_transaction).unwrap(),
        ))
        .await
        .map_err(error_handle_of_web3)?;

    Ok(Json(ApiResponse {
        success: true,
        code: 200,
        json: Some(TxAddressDto {
            tx_address: tx_address.to_string(),
        }),
        message: None,
    }))
}

#[openapi]
#[post("/contract/deployContract", data = "<body>")]
pub async fn deploy_contract(
    db: &State<Mongo>,
    vault: &State<Vault>,
    eth_node: &State<EthNode>,
    body: Json<DeployContractDto>,
) -> Result<Json<ApiResponse<ContractAddressDto>>, Json<ApiResponse<String>>> {
    let bytecode = include_str!("../../contract/bytecode.json");

    let res = vault
        .get_one_account(&body.account_name)
        .await
        .map_err(error_handle_of_reqwest)?;
    let data = Vec::from_hex(bytecode.to_string().replace("\"", "").replace("0x", "")).unwrap();
    let count = eth_node
        .get_transaction_count(&res.data.address.replace("0x", ""))
        .await
        .unwrap();
    let raw_tx = vault
        .create_one_raw_transaction("", count.low_u64(), data)
        .await;
    let signed_tx = vault
        .sign_one_transaction(&body.account_name, raw_tx)
        .await
        .map_err(error_handle_of_reqwest)?;
    let tx_address = eth_node
        .send_raw_transaction(Bytes::from(
            hex::decode(signed_tx.data.signed_transaction).unwrap(),
        ))
        .await
        .map_err(error_handle_of_web3)?;
    let contract_address = eth_node
        .wait_contract_address_of_transaction_receipt(tx_address)
        .await
        .map_err(error_handle_of_web3)?;

    let _ = insert_one_contract(db, "ProofOfExistence", contract_address, tx_address)
        .await
        .map_err(error_handle_of_string)?;
    Ok(Json(ApiResponse {
        success: true,
        code: 200,
        json: Some(ContractAddressDto {
            tx_address: Some(format!("{:?}", tx_address)),
            contract_address: Some(format!("{:?}", contract_address)),
        }),
        message: None,
    }))
}
