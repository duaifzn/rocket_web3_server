use crate::config::Config;
use crate::dto::role::Role;
use chrono::prelude::*;
use chrono::Duration;
use jsonwebtoken::Algorithm;
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use rocket::serde::{Deserialize, Serialize};
lazy_static! {
    static ref CONFIG: Config<'static> = Config::load();
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
struct Claims {
    exp: usize,
}

pub fn auth_token_generate(role: Role) -> String {
    let expire_time = Utc::now() + Duration::seconds(CONFIG.jwt_expire);
    let header = Header {
        kid: Some((role as u8).to_string()),
        alg: Algorithm::HS512,
        ..Default::default()
    };
    let claims = Claims {
        exp: expire_time.timestamp() as usize,
    };
    let token = encode(
        &header,
        &claims,
        &EncodingKey::from_secret(CONFIG.jwt_secret.as_ref()),
    )
    .unwrap();
    token
}

pub fn user_auth_token_is_valid(token: &str) -> bool {
    // `token` is a struct with 2 fields: `header` and `claims` where `claims` is your own struct.
    let new_token = token.clone().replace("Bearer ", "");
    let token_data = decode::<Claims>(
        &new_token,
        &DecodingKey::from_secret(CONFIG.jwt_secret.as_ref()),
        &Validation::new(Algorithm::HS512),
    );
    match token_data {
        Ok(data) => match data.header.kid.unwrap().parse::<u8>().unwrap() <= Role::User as u8 {
            true => return true,
            false => return false,
        },
        Err(_) => false,
    }
}

pub fn admin_auth_token_is_valid(token: &str) -> bool {
    // `token` is a struct with 2 fields: `header` and `claims` where `claims` is your own struct.
    let new_token = token.clone().replace("Bearer ", "");
    let token_data = decode::<Claims>(
        &new_token,
        &DecodingKey::from_secret(CONFIG.jwt_secret.as_ref()),
        &Validation::new(Algorithm::HS512),
    );
    match token_data {
        Ok(data) => match data.header.kid.unwrap().parse::<u8>().unwrap() <= Role::Admin as u8 {
            true => return true,
            false => return false,
        },
        Err(_) => false,
    }
}
