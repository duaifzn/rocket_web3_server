use crate::contract_interface::proof_of_existence_interface;
use crate::contract_interface::proof_of_existence_interface::ProofOfExistence;
use crate::database::Mongo;
use crate::dto::request_dto::{
    AddIssuerDto, DelIssuerDto, DeployContractDto, HashDto, NotarizeHashDto, RawDto, RevokeHashDto,
    TransferOwnershipDto,
};
use crate::dto::response_dto::{
    ApiResponse, BoolDto, ContractAddressDto, CustomContractLogDto, CustomTransactionReceiptDto,
    HashValueDto, SendHashDto, Sha256HashDto, TxAddressDto,
};
use crate::middleware::{admin_auth_guard, user_auth_guard};
use crate::service::contract_service::insert_one_contract;
use crate::util::error_handle::{
    error_handle_of_reqwest, error_handle_of_string, error_handle_of_web3,
};
use crate::util::eth_node::EthNode;
use crate::util::vault::Vault;
use hex::FromHex;
use rocket::http::Status;
use rocket::serde::json::Json;
use rocket::State;
use rocket_okapi::openapi;
use sha2::{Digest, Sha256};
use std::str::FromStr;
use uuid::Uuid;
use web3::ethabi::{FixedBytes, RawLog};
use web3::types::{BlockNumber, Bytes, FilterBuilder, H160, H256};

/// # Unuse
///
/// test route.
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

/// # Unuse
///
/// test route.
#[openapi]
#[post("/hash", format = "json", data = "<body>")]
pub async fn send_hash(
    eth_node: &State<EthNode>,
    body: Json<HashDto>,
) -> Json<ApiResponse<SendHashDto>> {
    let contract = eth_node.connect_contract_of_proof_of_existence(&body.address.to_owned()).await;
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

/// # Check if issuer_address is issuer or not
///
/// use isIssuer method of contract.
#[openapi]
#[get("/contract/isIssuer?<contract_address>&<account_name>&<issuer_address>")]
pub async fn is_issuer(
    _token: user_auth_guard::Token<'_>,
    vault: &State<Vault>,
    eth_node: &State<EthNode>,
    contract_address: String,
    account_name: String,
    issuer_address: String,
) -> Result<Json<ApiResponse<BoolDto>>, (Status, Json<ApiResponse<String>>)> {
    let contract = eth_node.connect_contract_of_proof_of_existence(&contract_address).await;
    let res = vault
        .get_one_account(&account_name)
        .await
        .map_err(error_handle_of_reqwest)?;

    let contract_address = EthNode::hex_str_to_bytes20(&contract_address.replace("0x", ""))
        .map_err(error_handle_of_web3)?;
    let account_address = EthNode::hex_str_to_bytes20(&res.data.address.replace("0x", ""))
        .map_err(error_handle_of_web3)?;
    let issuer_address = EthNode::hex_str_to_bytes20(&issuer_address.replace("0x", ""))
        .map_err(error_handle_of_web3)?;

    let data = contract
        .abi()
        .function("isIssuer")
        .unwrap()
        .encode_input(&[web3::ethabi::Token::Address(H160::from(issuer_address))])
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
        .read()
        .await
        .eth()
        .call(call_req, None)
        .await
        .map_err(error_handle_of_web3)?;
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

/// # Post key/value to contract
///
/// use notarizeHash method of contract.
#[openapi]
#[post("/contract/notarizeHash", format = "json", data = "<body>")]
pub async fn notarize_hash(
    _token: user_auth_guard::Token<'_>,
    db: &State<Mongo>,
    vault: &State<Vault>,
    eth_node: &State<EthNode>,
    body: Json<NotarizeHashDto>,
) -> Result<Json<ApiResponse<TxAddressDto>>, (Status, Json<ApiResponse<String>>)> {
    let contract = eth_node.connect_contract_of_proof_of_existence(&body.contract_address).await;
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

    let contract_address = EthNode::hex_str_to_bytes20(&body.contract_address.replace("0x", ""))
        .map_err(error_handle_of_web3)?;
    let account_address = EthNode::hex_str_to_bytes20(&res.data.address.replace("0x", ""))
        .map_err(error_handle_of_web3)?;
    let call_req = web3::types::CallRequest {
        from: Some(web3::types::Address::from(account_address)),
        to: Some(web3::types::Address::from(contract_address)),
        gas: None,
        gas_price: None,
        value: None,
        data: Some(Bytes(data.clone())),
        transaction_type: None,
        access_list: None,
        max_fee_per_gas: None,
        max_priority_fee_per_gas: None,
    };
    let _ = eth_node
        .web3
        .read()
        .await
        .eth()
        .call(call_req, None)
        .await
        .map_err(error_handle_of_web3)?;

    let count = eth_node
        .get_transaction_count(&res.data.address.replace("0x", ""))
        .await
        .map_err(error_handle_of_web3)?;
    let raw_tx = vault
        .create_one_raw_transaction(&body.contract_address, count.low_u64(), data)
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

/// # Get value by key
///
/// use getHash method of contract.
#[openapi]
#[get("/contract/getHash?<contract_address>&<account_name>&<key>")]
pub async fn get_hash(
    _token: user_auth_guard::Token<'_>,
    vault: &State<Vault>,
    eth_node: &State<EthNode>,
    contract_address: String,
    account_name: String,
    key: String,
) -> Result<Json<ApiResponse<HashValueDto>>, (Status, Json<ApiResponse<String>>)> {
    let contract = eth_node.connect_contract_of_proof_of_existence(&contract_address).await;
    let res = vault
        .get_one_account(&account_name)
        .await
        .map_err(error_handle_of_reqwest)?;

    let contract_address = EthNode::hex_str_to_bytes20(&contract_address.replace("0x", ""))
        .map_err(error_handle_of_web3)?;
    let account_address = EthNode::hex_str_to_bytes20(&res.data.address.replace("0x", ""))
        .map_err(error_handle_of_web3)?;
    let key_sha256 = EthNode::sha256_hash(&key);
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
        .read()
        .await
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

/// # Revoke value by key
///
/// use revokeHash method of contract.
#[openapi]
#[patch("/contract/revokeHash", format = "json", data = "<body>")]
pub async fn revoke_hash(
    _token: user_auth_guard::Token<'_>,
    db: &State<Mongo>,
    vault: &State<Vault>,
    eth_node: &State<EthNode>,
    body: Json<RevokeHashDto>,
) -> Result<Json<ApiResponse<TxAddressDto>>, (Status, Json<ApiResponse<String>>)> {
    let contract = eth_node.connect_contract_of_proof_of_existence(&body.contract_address).await;
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
        .create_one_raw_transaction(&body.contract_address, count.low_u64(), data)
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

/// # Check if key/value is revoked or not
///
/// use revokeHash method of contract.
#[openapi]
#[get("/contract/isRevoked?<contract_address>&<account_name>&<key>")]
pub async fn is_revoked(
    _token: user_auth_guard::Token<'_>,
    vault: &State<Vault>,
    eth_node: &State<EthNode>,
    contract_address: String,
    account_name: String,
    key: String,
) -> Result<Json<ApiResponse<BoolDto>>, (Status, Json<ApiResponse<String>>)> {
    let contract = eth_node.connect_contract_of_proof_of_existence(&contract_address).await;
    let res = vault
        .get_one_account(&account_name)
        .await
        .map_err(error_handle_of_reqwest)?;

    let contract_address = EthNode::hex_str_to_bytes20(&contract_address.replace("0x", ""))
        .map_err(error_handle_of_web3)?;
    let account_address = EthNode::hex_str_to_bytes20(&res.data.address.replace("0x", ""))
        .map_err(error_handle_of_web3)?;
    let key_sha256 = EthNode::sha256_hash(&key);
    let key = EthNode::hex_str_to_bytes32(&key_sha256).map_err(error_handle_of_web3)?;

    let data = contract
        .abi()
        .function("isRevoked")
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
        .read()
        .await
        .eth()
        .call(call_req, None)
        .await
        .map_err(error_handle_of_web3)?;

    let result_temp = contract
        .abi()
        .function("isRevoked")
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

/// # Add one issuer
///
/// use addIssuer method of contract.
#[openapi]
#[post("/contract/addIssuer", format = "json", data = "<body>")]
pub async fn add_issuer(
    _token: admin_auth_guard::Token<'_>,
    db: &State<Mongo>,
    vault: &State<Vault>,
    eth_node: &State<EthNode>,
    body: Json<AddIssuerDto>,
) -> Result<Json<ApiResponse<TxAddressDto>>, (Status, Json<ApiResponse<String>>)> {
    let contract = eth_node.connect_contract_of_proof_of_existence(&body.contract_address).await;
    let res = vault
        .get_one_account(&body.account_name)
        .await
        .map_err(error_handle_of_reqwest)?;

    let issuer_address = EthNode::hex_str_to_bytes20(&body.issuer_address.replace("0x", ""))
        .map_err(error_handle_of_web3)?;
    let data = contract
        .abi()
        .function("addIssuer")
        .unwrap()
        .encode_input(&[web3::ethabi::Token::Address(H160::from(issuer_address))])
        .unwrap();
    let count = eth_node
        .get_transaction_count(&res.data.address.replace("0x", ""))
        .await
        .map_err(error_handle_of_web3)?;
    let raw_tx = vault
        .create_one_raw_transaction(&body.contract_address, count.low_u64(), data)
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

/// # Delete one issuer
///
/// use delIssuer method of contract.
#[openapi]
#[delete("/contract/delIssuer", format = "json", data = "<body>")]
pub async fn del_issuer(
    _token: admin_auth_guard::Token<'_>,
    db: &State<Mongo>,
    vault: &State<Vault>,
    eth_node: &State<EthNode>,
    body: Json<DelIssuerDto>,
) -> Result<Json<ApiResponse<TxAddressDto>>, (Status, Json<ApiResponse<String>>)> {
    let contract = eth_node.connect_contract_of_proof_of_existence(&body.contract_address).await;
    let res = vault
        .get_one_account(&body.account_name)
        .await
        .map_err(error_handle_of_reqwest)?;
    let issuer_address = EthNode::hex_str_to_bytes20(&body.issuer_address.replace("0x", ""))
        .map_err(error_handle_of_web3)?;
    let data = contract
        .abi()
        .function("delIssuer")
        .unwrap()
        .encode_input(&[web3::ethabi::Token::Address(H160::from(issuer_address))])
        .unwrap();
    let count = eth_node
        .get_transaction_count(&res.data.address.replace("0x", ""))
        .await
        .map_err(error_handle_of_web3)?;
    let raw_tx = vault
        .create_one_raw_transaction(&body.contract_address, count.low_u64(), data)
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

/// # Transfer contract ownership
/// 
/// use transferOwnership method of contract.
#[openapi]
#[patch("/contract/transferOwnership", format = "json", data = "<body>")]
pub async fn transfer_ownership(
    _token: admin_auth_guard::Token<'_>,
    db: &State<Mongo>,
    vault: &State<Vault>,
    eth_node: &State<EthNode>,
    body: Json<TransferOwnershipDto>,
) -> Result<Json<ApiResponse<TxAddressDto>>, (Status, Json<ApiResponse<String>>)> {
    let contract = eth_node.connect_contract_of_proof_of_existence(&body.contract_address).await;
    let res = vault
        .get_one_account(&body.account_name)
        .await
        .map_err(error_handle_of_reqwest)?;
    let issuer_address = EthNode::hex_str_to_bytes20(&body.issuer_address.replace("0x", ""))
        .map_err(error_handle_of_web3)?;
    let data = contract
        .abi()
        .function("transferOwnership")
        .unwrap()
        .encode_input(&[web3::ethabi::Token::Address(H160::from(issuer_address))])
        .unwrap();
    let count = eth_node
        .get_transaction_count(&res.data.address.replace("0x", ""))
        .await
        .map_err(error_handle_of_web3)?;
    let raw_tx = vault
        .create_one_raw_transaction(&body.contract_address, count.low_u64(), data)
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

/// # Deploy one contract
/// 
/// use deployContract method of contract.
#[openapi]
#[post("/contract/deployContract", format = "json", data = "<body>")]
pub async fn deploy_contract(
    _token: admin_auth_guard::Token<'_>,
    db: &State<Mongo>,
    vault: &State<Vault>,
    eth_node: &State<EthNode>,
    body: Json<DeployContractDto>,
) -> Result<Json<ApiResponse<ContractAddressDto>>, (Status, Json<ApiResponse<String>>)> {
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

/// # Get one transaction log
/// 
/// get one transaction log from eth node.
#[openapi]
#[get("/contract/log/transaction?<contract_address>&<tx_address>")]
pub async fn get_one_transaction_log(
    _token: user_auth_guard::Token<'_>,
    eth_node: &State<EthNode>,
    contract_address: String,
    tx_address: String,
) -> Result<Json<ApiResponse<CustomTransactionReceiptDto>>, (Status, Json<ApiResponse<String>>)> {
    let contract = eth_node.connect_contract_of_proof_of_existence(&contract_address).await;
    let tx_address = EthNode::hex_str_to_bytes32(&tx_address.clone().replace("0x", ""))
        .map_err(error_handle_of_web3)?;
    let transaction_temp = eth_node
        .get_transaction(H256::from(tx_address))
        .await
        .map_err(error_handle_of_web3)?;
    let transaction = match transaction_temp {
        Some(result) => result,
        None => {
            return Ok(Json(ApiResponse {
                success: true,
                code: 200,
                json: None,
                message: Some("null transaction!".to_string()),
            }))
        }
    };
    let receipt_temp = eth_node
        .get_one_transaction_receipt(H256::from(tx_address))
        .await
        .map_err(error_handle_of_web3)?;

    let receipt = match receipt_temp {
        Some(result) => result,
        None => {
            return Ok(Json(ApiResponse {
                success: true,
                code: 200,
                json: None,
                message: Some("null receipt!".to_string()),
            }))
        }
    };
    let timestamp = match receipt.block_hash {
        Some(hash) => eth_node
            .get_blockhash_timestamp(hash)
            .await
            .map_err(error_handle_of_web3)?
            .unwrap(),
        None => {
            return Ok(Json(ApiResponse {
                success: true,
                code: 200,
                json: None,
                message: Some("null timestamp!".to_string()),
            }))
        }
    };
    let log_first = match receipt.logs.first() {
        Some(log) => log.to_owned(),
        None => {
            return Ok(Json(ApiResponse {
                success: true,
                code: 200,
                json: None,
                message: Some("null log!".to_string()),
            }))
        }
    };
    let (event_name, decoded_log) = EthNode::decode_log(
        contract.abi().events(),
        RawLog {
            topics: log_first.topics.clone(),
            data: log_first.data.0.clone(),
        },
    );

    match event_name {
        Some(_) => {}
        None => {
            return Ok(Json(ApiResponse {
                success: true,
                code: 200,
                json: None,
                message: Some("event not match!".to_string()),
            }))
        }
    };
    let decoded = ProofOfExistence::decode_event_log(event_name.unwrap().as_str(), decoded_log);

    Ok(Json(ApiResponse {
        success: true,
        code: 200,
        json: Some(CustomTransactionReceiptDto {
            blockhash: Some(format!("{:?}", receipt.block_hash.unwrap())),
            parenthash: None,
            nonce: Some(format!("{:?}", transaction.nonce)),
            tx_address: Some(format!("{:?}", receipt.transaction_hash)),
            decode_log: Some(decoded),
            timestamp: Some(format!("{:?}", timestamp)),
        }),
        message: None,
    }))
}

/// # Get all transaction log by one blockhash
/// 
/// get all transaction log by blockhash from eth node.
#[openapi]
#[get("/contract/log/blockhash?<contract_address>&<blockhash>")]
pub async fn get_blockhash_transactions_log(
    _token: user_auth_guard::Token<'_>,
    eth_node: &State<EthNode>,
    contract_address: String,
    blockhash: String,
) -> Result<Json<ApiResponse<Vec<CustomTransactionReceiptDto>>>, (Status, Json<ApiResponse<String>>)> {
    let contract = eth_node.connect_contract_of_proof_of_existence(&contract_address).await;
    let blockhash = EthNode::hex_str_to_bytes32(&blockhash.clone().replace("0x", ""))
        .map_err(error_handle_of_web3)?;
    let (transactions, timestamp_temp) = eth_node
        .get_all_transactions_of_blockhash(H256::from(blockhash))
        .await
        .map_err(error_handle_of_web3)?;
    let parent_hash = eth_node
        .get_blockhash_parent_hash(H256::from(blockhash))
        .await
        .map_err(error_handle_of_web3)?;
    let timestamp = match timestamp_temp {
        Some(tt) => tt,
        None => {
            return Ok(Json(ApiResponse {
                success: true,
                code: 200,
                json: None,
                message: Some("null timestamp!".to_string()),
            }))
        }
    };
    let txs = match transactions {
        Some(txs) => txs,
        None => {
            return Ok(Json(ApiResponse {
                success: true,
                code: 200,
                json: None,
                message: Some("None transaction in block!".to_string()),
            }))
        }
    };
    let receipts = eth_node.get_many_transaction_receipts(txs).await;
    let result: Vec<CustomTransactionReceiptDto> = receipts
        .into_iter()
        .map(|receipt_temp| {
            if receipt_temp == None {
                return CustomTransactionReceiptDto {
                    blockhash: None,
                    parenthash: None,
                    nonce: None,
                    tx_address: None,
                    decode_log: None,
                    timestamp: None,
                };
            } else {
                let receipt = receipt_temp.unwrap();
                let log_first = match receipt.logs.first() {
                    Some(log) => log.to_owned(),
                    None => {
                        return CustomTransactionReceiptDto {
                            blockhash: Some(format!("{:?}", receipt.block_hash.unwrap())),
                            parenthash: Some(format!("{:?}", parent_hash.unwrap())),
                            nonce: None,
                            tx_address: Some(format!("{:?}", receipt.transaction_hash)),
                            decode_log: None,
                            timestamp: Some(format!("{:?}", timestamp)),
                        }
                    }
                };
                let (event_name, decoded_log) = EthNode::decode_log(
                    contract.abi().events(),
                    RawLog {
                        topics: log_first.topics.clone(),
                        data: log_first.data.0.clone(),
                    },
                );
                let decoded =
                    ProofOfExistence::decode_event_log(event_name.unwrap().as_str(), decoded_log);
                return CustomTransactionReceiptDto {
                    blockhash: Some(format!("{:?}", receipt.block_hash.unwrap())),
                    parenthash: Some(format!("{:?}", parent_hash.unwrap())),
                    nonce: None,
                    tx_address: Some(format!("{:?}", receipt.transaction_hash)),
                    decode_log: Some(decoded),
                    timestamp: Some(format!("{:?}", timestamp)),
                };
            }
        })
        .collect();
    Ok(Json(ApiResponse {
        success: true,
        code: 200,
        json: Some(result),
        message: None,
    }))
}

/// # Get contract all log
/// 
/// get contract all log.
#[openapi]
#[get("/contract/log/all?<contract_address>")]
pub async fn get_all_log_of_proof_of_existence(
    // _token: user_auth_guard::Token<'_>,
    eth_node: &State<EthNode>,
    contract_address: String,
) -> Result<Json<ApiResponse<Vec<CustomContractLogDto>>>, (Status, Json<ApiResponse<String>>)> {
    let contract = eth_node.connect_contract_of_proof_of_existence(&contract_address).await;
    let filter = FilterBuilder::default()
        .from_block(BlockNumber::Earliest)
        .to_block(BlockNumber::Latest)
        .address(vec![contract.address()])
        .build();

    let filter = eth_node
        .web3
        .read()
        .await
        .eth_filter()
        .create_logs_filter(filter)
        .await
        .map_err(error_handle_of_web3)?;

    let logs = filter.logs().await.map_err(error_handle_of_web3)?;
    let contract_logs: Vec<CustomContractLogDto> = logs
        .into_iter()
        .map(|log| {
            let (event_name, decoded_log) = EthNode::decode_log(
                contract.abi().events(),
                RawLog {
                    topics: log.topics.clone(),
                    data: log.data.0.clone(),
                },
            );
            if event_name.is_none() {
                return CustomContractLogDto {
                    blockhash: None,
                    tx_address: None,
                    contract_address: None,
                    decode_log: None,
                };
            }
            let decoded =
                ProofOfExistence::decode_event_log(event_name.unwrap().as_str(), decoded_log);
            return CustomContractLogDto {
                blockhash: Some(format!("{:?}", log.block_hash.unwrap())),
                tx_address: Some(format!("{:?}", log.transaction_hash.unwrap())),
                contract_address: Some(format!("{:?}", log.address)),
                decode_log: Some(decoded),
            };
        })
        .collect();
    Ok(Json(ApiResponse {
        success: true,
        code: 200,
        json: Some(contract_logs),
        message: None,
    }))
}

/// # Get contract log by custom timestamp
/// 
/// get contract log by custom timestamp.
#[openapi]
#[get(
    "/contract/log/event/ProofCreated?<contract_address>&<key>&<start_timestamp>&<end_timestamp>"
)]
pub async fn get_proof_created_log_of_proof_of_existence(
    // _token: user_auth_guard::Token<'_>,
    eth_node: &State<EthNode>,
    contract_address: String,
    key: String,
    start_timestamp: Option<u128>,
    end_timestamp: Option<u128>,
) -> Result<Json<ApiResponse<Vec<CustomContractLogDto>>>, (Status, Json<ApiResponse<String>>)> {
    let contract = eth_node.connect_contract_of_proof_of_existence(&contract_address).await;
    let key_sha256 = EthNode::sha256_hash(&key.clone()).to_lowercase();
    let filter = FilterBuilder::default()
        .from_block(BlockNumber::Earliest)
        .to_block(BlockNumber::Latest)
        .address(vec![contract.address()])
        .topics(
            Some(vec![contract
                .abi()
                .event("ProofCreated")
                .unwrap()
                .signature()]),
            None,
            None,
            None,
        )
        .build();

    let filter = eth_node
        .web3
        .read()
        .await
        .eth_filter()
        .create_logs_filter(filter)
        .await
        .map_err(error_handle_of_web3)?;

    let logs = filter.logs().await.map_err(error_handle_of_web3)?;
    let contract_logs_temp: Vec<CustomContractLogDto> = logs
        .into_iter()
        .map(|log| {
            let (event_name, decoded_log) = EthNode::decode_log(
                contract.abi().events(),
                RawLog {
                    topics: log.topics.clone(),
                    data: log.data.0.clone(),
                },
            );
            if event_name.is_none() {
                return CustomContractLogDto {
                    blockhash: None,
                    tx_address: None,
                    contract_address: None,
                    decode_log: None,
                };
            }
            let decoded =
                ProofOfExistence::decode_event_log(event_name.unwrap().as_str(), decoded_log);
            match decoded.key == Some(key_sha256.to_owned()) {
                true => {
                    return CustomContractLogDto {
                        blockhash: Some(format!("{:?}", log.block_hash.unwrap())),
                        tx_address: Some(format!("{:?}", log.transaction_hash.unwrap())),
                        contract_address: Some(format!("{:?}", log.address)),
                        decode_log: Some(decoded),
                    }
                }
                false => {
                    return CustomContractLogDto {
                        blockhash: None,
                        tx_address: None,
                        contract_address: None,
                        decode_log: None,
                    }
                }
            }
        })
        .filter(|dto| dto.tx_address.is_some())
        .collect();
    let mut contract_logs: Vec<CustomContractLogDto> = vec![];

    if start_timestamp.is_some() && end_timestamp.is_some() {
        for contract_log in contract_logs_temp {
            let t = eth_node
                .get_blockhash_timestamp(
                    H256::from_str(&contract_log.blockhash.clone().unwrap()).unwrap(),
                )
                .await
                .map_err(error_handle_of_web3)?
                .unwrap();
            let block_time = format!("{:?}", t).parse::<u128>().unwrap();
            if block_time >= start_timestamp.unwrap() && block_time <= end_timestamp.unwrap() {
                contract_logs.push(contract_log)
            }
        }
    } else {
        contract_logs = contract_logs_temp;
    }
    Ok(Json(ApiResponse {
        success: true,
        code: 200,
        json: Some(contract_logs),
        message: None,
    }))
}
