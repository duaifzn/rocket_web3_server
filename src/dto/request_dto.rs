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
    pub address: String,
    pub account_name: String,
    pub key: String,
    pub value: String
}

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
#[serde(crate = "rocket::serde")]
pub struct IsIssuerDto{
    pub address: String,
    pub account_name: String,
    pub target_address: String
}

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
#[serde(crate = "rocket::serde")]
pub struct GetHashDto{
    pub address: String,
    pub account_name: String,
    pub key: String
}

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
#[serde(crate = "rocket::serde")]
pub struct RevokeHashDto{
    pub address: String,
    pub account_name: String,
    pub key: String
}

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
#[serde(crate = "rocket::serde")]
pub struct AddIssuerDto{
    pub address: String,
    pub account_name: String,
    pub target_address: String
}

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
#[serde(crate = "rocket::serde")]
pub struct DelIssuerDto{
    pub address: String,
    pub account_name: String,
    pub target_address: String
}

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
#[serde(crate = "rocket::serde")]
pub struct TransferOwnershipDto{
    pub address: String,
    pub account_name: String,
    pub target_address: String
}