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
pub async fn signup_one_user(db: &State<Mongo>,user: Json<UserDto>) -> Json<ApiResponse<CreateOneUserDto>>{
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

#[openapi]
#[get("/hash", data="<raw>")]
pub async fn sha256_hash(raw: Json<RawDto>) ->Json<ApiResponse<Sha256HashDto>>{
    let mut hasher = Sha256::new();
    hasher.update(raw.raw_data.as_bytes());
    let done = hasher.finalize();
    let hash_data = format!("{:X}", done);
    Json(ApiResponse{
            success: true,
            code: 200,
            json: Some(Sha256HashDto{
                hash_data: hash_data
            }),
            message: None
        })
}

#[openapi]
#[post("/hash", data="<hash>")]
pub async fn send_hash(eth_node: &State<EthNode>, hash: Json<HashDto>) ->Json<ApiResponse<SendHashDto>>{
    let contract = eth_node.connect_contract_of_proof_of_existence(&hash.address.to_owned()).await;
    let contract_interface = ProofOfExistence{
        contract: contract
    };

    let new_uuid = Uuid::new_v4().to_string();
    let mut hasher = Sha256::new();
    hasher.update(new_uuid.as_bytes());
    let done = hasher.finalize();
    let new_uuid_sha256 = format!("{:X}", done);

    let response = contract_interface.notarize_hash(
        &hash.private_key.to_owned(),
        &new_uuid_sha256, 
        &hash.hash_data.to_owned()).await;
    match response{
        Ok(result) => Json(ApiResponse{
                success: true,
                code: 200,
                json: Some(SendHashDto{
                    tx_hash: format!("{:#x}", result)
                }),
                message: None
            }),
        Err(err) => Json(ApiResponse{
            success: false,
            code: 400,
            json: None,
            message: Some(err.to_string())
        }),
    }

}