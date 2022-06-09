use rocket::serde::{Serialize, Deserialize};
use schemars::JsonSchema;

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
#[serde(crate = "rocket::serde")]
pub struct UserDto{
    pub email: String,
    pub password: String,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
#[serde(crate = "rocket::serde")]
pub struct RawDto{
    pub raw_data: String,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
#[serde(crate = "rocket::serde")]
pub struct HashDto{
    pub address: String,
    pub private_key: String,
    pub hash_data: String,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
#[serde(crate = "rocket::serde")]
pub struct NotarizeHashDto{
    pub contract_address: String,
    pub account_name: String,
    pub key: String,
    pub value: String
}

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
#[serde(crate = "rocket::serde")]
pub struct IsIssuerDto{
    pub contract_address: String,
    pub account_name: String,
    pub issuer_address: String
}

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
#[serde(crate = "rocket::serde")]
pub struct GetHashDto{
    pub contract_address: String,
    pub account_name: String,
    pub key: String
}

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
#[serde(crate = "rocket::serde")]
pub struct IsRevokeDto{
    pub contract_address: String,
    pub account_name: String,
    pub key: String
}

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
#[serde(crate = "rocket::serde")]
pub struct RevokeHashDto{
    pub contract_address: String,
    pub account_name: String,
    pub key: String
}

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
#[serde(crate = "rocket::serde")]
pub struct AddIssuerDto{
    pub contract_address: String,
    pub account_name: String,
    pub issuer_address: String
}

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
#[serde(crate = "rocket::serde")]
pub struct DelIssuerDto{
    pub contract_address: String,
    pub account_name: String,
    pub issuer_address: String
}

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
#[serde(crate = "rocket::serde")]
pub struct TransferOwnershipDto{
    pub contract_address: String,
    pub account_name: String,
    pub issuer_address: String
}

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
#[serde(crate = "rocket::serde")]
pub struct DeployContractDto{
    pub account_name: String,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
#[serde(crate = "rocket::serde")]
pub struct MoneyTransferDto{
    pub to_account_name: String
}

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
#[serde(crate = "rocket::serde")]
pub struct GetAccountBalanceDto{
    pub account_name: String
}


#[derive(Debug, Serialize, Deserialize, JsonSchema)]
#[serde(crate = "rocket::serde")]
pub struct GetBlockhashTxsDto{
    pub contract_address: String,
    pub blockhash: String
}

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
#[serde(crate = "rocket::serde")]
pub struct GetOneTransactionDto{
    pub contract_address: String,
    pub tx_address: String
}

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
#[serde(crate = "rocket::serde")]
pub struct GetContractAllLogDto{
    pub contract_address: String,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
#[serde(crate = "rocket::serde")]
pub struct GetContractLogDto{
    pub contract_address: String,
    pub key: String,
    pub start_timestamp: Option<u128>,
    pub end_timestamp: Option<u128>,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
#[serde(crate = "rocket::serde")]
pub struct NotarizeHashByKeyHashDto{
    pub email: String,
    pub hash: String,
}