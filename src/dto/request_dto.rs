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