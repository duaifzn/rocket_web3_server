use rocket::State;
use rocket::serde::json::Json;
use crate::dto::request_dto::{UserDto, RawDto, HashDto};
use crate::database::Mongo;
use crate::service::user_service::{insert_one_user, verify_one_user, find_one_user};
use crate::dto::response_dto::{ApiResponse, CreateOneUserDto, SigninOneUserDto, Sha256HashDto, SendHashDto};
use crate::util::auth::{auth_token_generate};
use crate::util::eth_node::EthNode;
use crate::middleware::auth_guard::Token;
use crate::contract_interface::proof_of_existence_interface::ProofOfExistence;
use crate::util::vault::Vault;
use sha2::{Sha256, Digest};
use uuid::Uuid;
use rocket_okapi::{openapi};

#[openapi]
#[get("/")]
pub fn index(token: Token<'_>) -> &'static str {
    "Hello, world!"
}
#[openapi]
#[post("/signup", data="<user>")]
pub async fn signup_one_user(db: &State<Mongo>, vault: &State<Vault>, user: Json<UserDto>) -> Json<ApiResponse<CreateOneUserDto>>{
    let is_duplicate = find_one_user(db, &user).await;
    match is_duplicate{
        Ok(result) =>{ 
            if result{
                return Json(ApiResponse{
                    success: false,
                    code: 401,
                    json: None,
                    message: Some("user duplicate!".to_string())
                })
            }
        },
        Err(err) => return Json(ApiResponse{
            success: false,
            code: 500,
            json: None,
            message: Some(err.to_string())
        })
    }
    let vault_res = vault.create_one_account(&user.email.clone()).await;
    println!("{:?}", vault_res);
    match vault_res{
        Ok(_) => {},
        Err(err) => return Json(ApiResponse{
            success: false,
            code: 500,
            json: None,
            message: Some(err.to_string())
        })
    }
    let data = insert_one_user(db, user).await;
    match data{
        Ok(result) => Json(ApiResponse{
            success: true,
            code: 200,
            json: Some(CreateOneUserDto{
                id: result.inserted_id.as_object_id().unwrap().to_string()
            }),
            message: None
        }),
        Err(err) => Json(ApiResponse{
            success: false,
            code: 500,
            json: None,
            message: Some(err.to_string())
        })
    }
}

#[openapi]
#[post("/signin", data="<user>")]
pub async fn signin_one_user(db: &State<Mongo>,user: Json<UserDto>) -> Json<ApiResponse<SigninOneUserDto>>{
    let data = verify_one_user(db, user).await;
    match data{
        Ok(result) => {
            if result{
                let token = auth_token_generate();
                return Json(ApiResponse{
                    success: true,
                    code: 200,
                    json: Some(SigninOneUserDto{
                        token: token
                    }),
                    message: None
                })
            }
            else{
                return Json(ApiResponse{
                    success: false,
                    code: 401,
                    json: None,
                    message: Some("verify error!".to_string())
                })
            }
        },
        Err(err) => Json(ApiResponse{
            success: false,
            code: 500,
            json: None,
            message: Some(err.to_string())
        })
    }
}