use std::io::Cursor;
use rocket::serde::{Serialize, Deserialize, json};
use rocket::response;
use rocket::request::Request;
use rocket::response::{Responder, Response};
use rocket::http::{ContentType};
use schemars::JsonSchema;

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
#[serde(crate = "rocket::serde")]
pub struct ApiResponse<T>{
    pub success: bool,
    pub code: u16,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub json: Option<T>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>
}

impl<'r, T: Serialize> Responder<'r, 'static> for ApiResponse<T>{
    fn respond_to(self, req: &Request) -> response::Result<'static> {
        let string = json::serde_json::to_string(&self).unwrap();
        Response::build()
            .sized_body(None, Cursor::new(string))
            .header(ContentType::JSON)
            .ok()
    }
}

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
#[serde(crate = "rocket::serde")]
pub struct CreateOneUserDto{
    pub id: String,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
#[serde(crate = "rocket::serde")]
pub struct SigninOneUserDto{
    pub token: String,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
#[serde(crate = "rocket::serde")]
pub struct Sha256HashDto{
    pub hash_data: String,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
#[serde(crate = "rocket::serde")]
pub struct SendHashDto{
    pub tx_hash: String,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
#[serde(crate = "rocket::serde")]
pub struct VaultAccountDto{
    pub request_id: String,
    pub lease_id: String,
    pub renewable: bool,
    pub lease_duration: u32,
    pub data: VaultAccountDataDto,
    pub wrap_info: Option<String>,
    pub warnings: Option<String>,
    pub auth: Option<String>
}

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
#[serde(crate = "rocket::serde")]
pub struct VaultAccountDataDto{
    pub address: String,
    pub name: String
}

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
#[serde(crate = "rocket::serde")]
pub struct VaultSignDto{
    pub request_id: String,
    pub lease_id: String,
    pub renewable: bool,
    pub lease_duration: u32,
    pub data: VaultSignDataDto,
    pub wrap_info: Option<String>,
    pub warnings: Option<String>,
    pub auth: Option<String>
}

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
#[serde(crate = "rocket::serde")]
pub struct VaultSignDataDto{
    pub from_address: String,
    pub name: String,
    pub signed_transaction: String,
    pub to_address: Option<String>
}

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
#[serde(crate = "rocket::serde")]
pub struct TxAddressDto{
    pub tx_address: String,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
#[serde(crate = "rocket::serde")]
pub struct BoolDto{
    pub result: bool,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
#[serde(crate = "rocket::serde")]
pub struct HashValueDto{
    pub hash_value: String,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
#[serde(crate = "rocket::serde")]
pub struct ContractAddressDto{
    pub tx_address: Option<String>,
    pub contract_address: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
#[serde(crate = "rocket::serde")]
pub struct AddressBalanceDto{
    pub account_address: Option<String>,
    pub balance: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
#[serde(crate = "rocket::serde")]
pub struct DecodeEventLogDto{
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub previous_owner: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub new_owner: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub issuer_account: Option<String>,
}
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
#[serde(crate = "rocket::serde")]
pub struct CustomTransactionReceiptDto{
    #[serde(skip_serializing_if = "Option::is_none")]
    pub blockhash: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parenthash: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nonce: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tx_address: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub decode_log: Option<DecodeEventLogDto>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timestamp: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
#[serde(crate = "rocket::serde")]
pub struct CustomContractLogDto{
    #[serde(skip_serializing_if = "Option::is_none")]
    pub blockhash: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tx_address: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contract_address: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub decode_log: Option<DecodeEventLogDto>,
}