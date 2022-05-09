
use rocket::serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct RawTransaction{
    pub address_to: String,
    pub data: String,
    pub encoding: String,
    pub amount: String,
    pub nonce: String,
    pub gas_limit: String,
    pub gas_price: String,
    pub chainID: String,
}