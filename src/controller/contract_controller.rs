use crate::contract_interface::proof_of_existence_interface;
use crate::database::Mongo;
use crate::dto::request_dto::{
    AddIssuerDto, DelIssuerDto, GetHashDto, HashDto, IsIssuerDto, NotarizeHashDto, RawDto,
    RevokeHashDto, TransferOwnershipDto, UserDto,
};
use crate::dto::response_dto::{
    ApiResponse, CreateOneUserDto, SendHashDto, Sha256HashDto, SigninOneUserDto,
};
use crate::middleware::auth_guard::Token;
use crate::util::eth_node::{self, EthNode};
use crate::util::vault::Vault;
use rocket::serde::json::Json;
use rocket::State;
use rocket_okapi::openapi;
use sha2::{Digest, Sha256};
use uuid::Uuid;
use web3::contract::Options;
use web3::types::{Bytes, H160, H256};

#[openapi]
#[get("/hash", data = "<raw>")]
pub async fn sha256_hash(raw: Json<RawDto>) -> Json<ApiResponse<Sha256HashDto>> {
    let mut hasher = Sha256::new();
    hasher.update(raw.raw_data.as_bytes());
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
#[post("/hash", data = "<hash>")]
pub async fn send_hash(
    eth_node: &State<EthNode>,
    hash: Json<HashDto>,
) -> Json<ApiResponse<SendHashDto>> {
    let contract = eth_node
        .connect_contract_of_proof_of_existence(&hash.address.to_owned())
        .await;
    let contract_interface = proof_of_existence_interface::ProofOfExistence { contract: contract };

    let new_uuid = Uuid::new_v4().to_string();
    let mut hasher = Sha256::new();
    hasher.update(new_uuid.as_bytes());
    let done = hasher.finalize();
    let new_uuid_sha256 = format!("{:X}", done);

    let response = contract_interface
        .notarize_hash(
            &hash.private_key.to_owned(),
            &new_uuid_sha256,
            &hash.hash_data.to_owned(),
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
#[post("/contract/isIssuer", data = "<hash>")]
pub async fn is_issuer(vault: &State<Vault>, eth_node: &State<EthNode>, hash: Json<IsIssuerDto>) {
    let contract = eth_node.connect_contract_of_proof_of_existence_by_vault(&hash.address);
    let res = vault.get_one_account(&hash.account_name).await.unwrap();
    let target_address =
        EthNode::hex_str_to_bytes20(&hash.target_address.replace("0x", "")).unwrap();
    let data = contract
        .abi()
        .function("isIssuer")
        .unwrap()
        .encode_input(&[web3::ethabi::Token::Address(H160::from(target_address))])
        .unwrap();
    let nonce = eth_node
        .get_transaction_nonce(&res.data.address)
        .await
        .unwrap();
    let raw_tx = vault
        .create_one_raw_transaction(&hash.address, nonce + 1, data)
        .await;
    let signed_tx = vault
        .sign_one_transaction(&hash.account_name, raw_tx)
        .await
        .unwrap();
    let tx_address = eth_node
        .send_raw_transaction(Bytes::from(
            hex::decode(signed_tx.data.signed_transaction).unwrap(),
        ))
        .await;
    // let a = eth_node.get_transaction(
    //     "0x585d8b145bbc41b9f3a07e89676cf05a1e67c18f58a5a044127005103b45cce6"
    //     ).await.unwrap();
    //0xbfc20bfcb201e2576c848991f18190363ff95b1029dc68751b2fb636cd042632
    // let account = eth_node::EthNode::hex_str_to_bytes20(&res.data.address).unwrap();
    // let a = contract_interface.is_issuer(H160::from(account)).await;
}

#[openapi]
#[post("/contract/notarizeHash", data = "<hash>")]
pub async fn notarize_hash(
    vault: &State<Vault>,
    eth_node: &State<EthNode>,
    hash: Json<NotarizeHashDto>,
) {
    let contract = eth_node.connect_contract_of_proof_of_existence_by_vault(&hash.address);
    let res = vault.get_one_account(&hash.account_name).await.unwrap();
    let key = EthNode::hex_str_to_bytes32(&hash.key).unwrap();
    let value = EthNode::hex_str_to_bytes32(&hash.value).unwrap();
    let data = contract
        .abi()
        .function("notarizeHash")
        .unwrap()
        .encode_input(&[
            web3::ethabi::Token::FixedBytes(key.to_vec()),
            web3::ethabi::Token::FixedBytes(value.to_vec()),
        ])
        .unwrap();
    let nonce = eth_node.get_transaction_nonce(&hash.address).await.unwrap();
    let raw_tx = vault
        .create_one_raw_transaction(&hash.address, nonce, data)
        .await;
    let signed_tx = vault
        .sign_one_transaction(&res.data.address, raw_tx)
        .await
        .unwrap();
    let tx_address = eth_node
        .send_raw_transaction(Bytes::from(
            hex::decode(signed_tx.data.signed_transaction).unwrap(),
        ))
        .await;
}

#[openapi]
#[post("/contract/getHash", data = "<hash>")]
pub async fn get_hash(vault: &State<Vault>, eth_node: &State<EthNode>, hash: Json<GetHashDto>) {
    let contract = eth_node.connect_contract_of_proof_of_existence_by_vault(&hash.address);
    let res = vault.get_one_account(&hash.account_name).await.unwrap();
    let key = EthNode::hex_str_to_bytes32(&hash.key).unwrap();
    let data = contract
        .abi()
        .function("getHash")
        .unwrap()
        .encode_input(&[web3::ethabi::Token::FixedBytes(key.to_vec())])
        .unwrap();
    let nonce = eth_node.get_transaction_nonce(&hash.address).await.unwrap();
    let raw_tx = vault
        .create_one_raw_transaction(&hash.address, nonce, data)
        .await;
    let signed_tx = vault
        .sign_one_transaction(&res.data.address, raw_tx)
        .await
        .unwrap();
    let tx_address = eth_node
        .send_raw_transaction(Bytes::from(
            hex::decode(signed_tx.data.signed_transaction).unwrap(),
        ))
        .await;
}

#[openapi]
#[post("/contract/revokeHash", data = "<hash>")]
pub async fn revoke_hash(
    vault: &State<Vault>,
    eth_node: &State<EthNode>,
    hash: Json<RevokeHashDto>,
) {
    let contract = eth_node.connect_contract_of_proof_of_existence_by_vault(&hash.address);
    let res = vault.get_one_account(&hash.account_name).await.unwrap();
    let key = EthNode::hex_str_to_bytes32(&hash.key).unwrap();
    let data = contract
        .abi()
        .function("revokeHash")
        .unwrap()
        .encode_input(&[web3::ethabi::Token::FixedBytes(key.to_vec())])
        .unwrap();
    let nonce = eth_node.get_transaction_nonce(&hash.address).await.unwrap();
    let raw_tx = vault
        .create_one_raw_transaction(&hash.address, nonce, data)
        .await;
    let signed_tx = vault
        .sign_one_transaction(&res.data.address, raw_tx)
        .await
        .unwrap();
    let tx_address = eth_node
        .send_raw_transaction(Bytes::from(
            hex::decode(signed_tx.data.signed_transaction).unwrap(),
        ))
        .await;
}

#[openapi]
#[post("/contract/addIssuer", data = "<hash>")]
pub async fn add_issuer(vault: &State<Vault>, eth_node: &State<EthNode>, hash: Json<AddIssuerDto>) {
    let contract = eth_node.connect_contract_of_proof_of_existence_by_vault(&hash.address);
    let res = vault.get_one_account(&hash.account_name).await.unwrap();
    let target_address =
        EthNode::hex_str_to_bytes32(&hash.target_address.replace("0x", "")).unwrap();
    let data = contract
        .abi()
        .function("addIssuer")
        .unwrap()
        .encode_input(&[web3::ethabi::Token::FixedBytes(target_address.to_vec())])
        .unwrap();
    let nonce = eth_node.get_transaction_nonce(&hash.address).await.unwrap();
    let raw_tx = vault
        .create_one_raw_transaction(&hash.address, nonce, data)
        .await;
    let signed_tx = vault
        .sign_one_transaction(&res.data.address, raw_tx)
        .await
        .unwrap();
    let tx_address = eth_node
        .send_raw_transaction(Bytes::from(
            hex::decode(signed_tx.data.signed_transaction).unwrap(),
        ))
        .await;
}

#[openapi]
#[post("/contract/delIssuer", data = "<hash>")]
pub async fn del_issuer(vault: &State<Vault>, eth_node: &State<EthNode>, hash: Json<DelIssuerDto>) {
    let contract = eth_node.connect_contract_of_proof_of_existence_by_vault(&hash.address);
    let res = vault.get_one_account(&hash.account_name).await.unwrap();
    let target_address =
        EthNode::hex_str_to_bytes32(&hash.target_address.replace("0x", "")).unwrap();
    let data = contract
        .abi()
        .function("delIssuer")
        .unwrap()
        .encode_input(&[web3::ethabi::Token::FixedBytes(target_address.to_vec())])
        .unwrap();
    let nonce = eth_node.get_transaction_nonce(&hash.address).await.unwrap();
    let raw_tx = vault
        .create_one_raw_transaction(&hash.address, nonce, data)
        .await;
    let signed_tx = vault
        .sign_one_transaction(&res.data.address, raw_tx)
        .await
        .unwrap();
    let tx_address = eth_node
        .send_raw_transaction(Bytes::from(
            hex::decode(signed_tx.data.signed_transaction).unwrap(),
        ))
        .await;
}

#[openapi]
#[post("/contract/transferOwnership", data = "<hash>")]
pub async fn transfer_ownership(
    vault: &State<Vault>,
    eth_node: &State<EthNode>,
    hash: Json<TransferOwnershipDto>,
) {
    let contract = eth_node.connect_contract_of_proof_of_existence_by_vault(&hash.address);
    let res = vault.get_one_account(&hash.account_name).await.unwrap();
    let target_address =
        EthNode::hex_str_to_bytes32(&hash.target_address.replace("0x", "")).unwrap();
    let data = contract
        .abi()
        .function("delIssuer")
        .unwrap()
        .encode_input(&[web3::ethabi::Token::FixedBytes(target_address.to_vec())])
        .unwrap();
    let nonce = eth_node.get_transaction_nonce(&hash.address).await.unwrap();
    let raw_tx = vault
        .create_one_raw_transaction(&hash.address, nonce, data)
        .await;
    let signed_tx = vault
        .sign_one_transaction(&res.data.address, raw_tx)
        .await
        .unwrap();
    let tx_address = eth_node
        .send_raw_transaction(Bytes::from(
            hex::decode(signed_tx.data.signed_transaction).unwrap(),
        ))
        .await;
}
