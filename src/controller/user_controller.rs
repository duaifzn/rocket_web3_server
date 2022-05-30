use crate::database::Mongo;
use crate::dto::request_dto::{
    GetAccountBalanceDto, UserDto,
};
use crate::dto::response_dto::{
    AddressBalanceDto, ApiResponse, CreateOneUserDto, SigninOneUserDto,
};
use crate::dto::role::Role;
use crate::middleware::user_auth_guard::Token;
use crate::service::user_service::{find_one_user, insert_one_user, verify_one_user};
use crate::util::auth::auth_token_generate;
use crate::util::error_handle::{error_handle_of_reqwest, error_handle_of_web3};
use crate::util::eth_node::EthNode;
use crate::util::vault::Vault;
use rocket::serde::json::Json;
use rocket::State;
use rocket_okapi::openapi;
use std::str::FromStr;
use web3::types::Address;

#[openapi]
#[get("/")]
pub fn index(_token: Token<'_>) -> &'static str {
    "Hello, world!"
}
#[openapi]
#[post("/signup", data = "<body>")]
pub async fn signup_one_user(
    db: &State<Mongo>,
    vault: &State<Vault>,
    body: Json<UserDto>,
) -> Json<ApiResponse<CreateOneUserDto>> {
    let is_duplicate = find_one_user(db, &body).await;
    match is_duplicate {
        Ok(result) => {
            if result {
                return Json(ApiResponse {
                    success: false,
                    code: 401,
                    json: None,
                    message: Some("user duplicate!".to_string()),
                });
            }
        }
        Err(err) => {
            return Json(ApiResponse {
                success: false,
                code: 500,
                json: None,
                message: Some(err.to_string()),
            })
        }
    }
    let vault_res = vault.create_one_account(&body.email.clone()).await;
    match vault_res {
        Ok(_) => {}
        Err(err) => {
            return Json(ApiResponse {
                success: false,
                code: 500,
                json: None,
                message: Some(err.to_string()),
            })
        }
    }
    let data = insert_one_user(db, body).await;
    match data {
        Ok(result) => Json(ApiResponse {
            success: true,
            code: 200,
            json: Some(CreateOneUserDto {
                id: result.inserted_id.as_object_id().unwrap().to_string(),
            }),
            message: None,
        }),
        Err(err) => Json(ApiResponse {
            success: false,
            code: 500,
            json: None,
            message: Some(err.to_string()),
        }),
    }
}

#[openapi]
#[post("/signin", data = "<body>")]
pub async fn signin_one_user(
    db: &State<Mongo>,
    body: Json<UserDto>,
) -> Json<ApiResponse<SigninOneUserDto>> {
    let data = verify_one_user(db, body).await;
    match data {
        Ok((result, user)) => {
            if result {
                let role = Role::from_u8(user.unwrap().role);
                let token = auth_token_generate(role);
                return Json(ApiResponse {
                    success: true,
                    code: 200,
                    json: Some(SigninOneUserDto { token: token }),
                    message: None,
                });
            } else {
                return Json(ApiResponse {
                    success: false,
                    code: 401,
                    json: None,
                    message: Some("verify error!".to_string()),
                });
            }
        }
        Err(err) => Json(ApiResponse {
            success: false,
            code: 500,
            json: None,
            message: Some(err.to_string()),
        }),
    }
}

#[openapi]
#[get("/balance", data = "<body>")]
pub async fn get_account_balance(
    _token: Token<'_>,
    eth_node: &State<EthNode>,
    vault: &State<Vault>,
    body: Json<GetAccountBalanceDto>,
) -> Result<Json<ApiResponse<AddressBalanceDto>>, Json<ApiResponse<String>>> {
    let res = vault
        .get_one_account(&body.account_name)
        .await
        .map_err(error_handle_of_reqwest)?;
    let address = Address::from_str(&res.data.address.replace("0x", "")).unwrap();
    let balance = eth_node
        .get_account_balance(address)
        .await
        .map_err(error_handle_of_web3)?;

    Ok(Json(ApiResponse {
        success: true,
        code: 200,
        json: Some(AddressBalanceDto {
            account_address: Some(res.data.address),
            balance: Some(format!("{:?}", balance)),
        }),
        message: None,
    }))
}
