use jsonwebtoken::{encode, decode, Header, Validation, EncodingKey, DecodingKey};
use rocket::serde::{Serialize, Deserialize};
use chrono::prelude::*;
use chrono::Duration;
use crate::config::Config;
lazy_static!{
    static ref CONFIG: Config<'static> = Config::load();
}
#[derive(Debug, Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
struct Claims {
    exp: usize,
}

pub fn auth_token_generate() ->String{
    let expire_time = Utc::now() + Duration::seconds(CONFIG.jwt_expire);
    let claims = Claims{
        exp: expire_time.timestamp() as usize,
    };
    let token = encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(CONFIG.jwt_secret.as_ref())).unwrap();
    token
}

pub fn auth_token_is_valid(token: &str) ->bool{
    // `token` is a struct with 2 fields: `header` and `claims` where `claims` is your own struct.
    let token_data = decode::<Claims>(token,
        &DecodingKey::from_secret(CONFIG.jwt_secret.as_ref()),
        &Validation::default());
    match token_data{
        Ok(_) => true,
        Err(_) => false
    }
}