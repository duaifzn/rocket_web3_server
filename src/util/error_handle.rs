use reqwest;
use web3;
use rocket::serde::json::Json;
use crate::dto::response_dto::ApiResponse;

pub fn error_handle_of_string(error: String) ->Json<ApiResponse<String>>{
    println!("{:?}", error);
    Json(ApiResponse {
        success: false,
        code: 500,
        json: None,
        message: Some(error),
    })
}

pub fn error_handle_of_reqwest(error: reqwest::Error) ->Json<ApiResponse<String>>{
    println!("{:?}", error);
    Json(ApiResponse {
        success: false,
        code: 500,
        json: None,
        message: Some(format!("{:?}", error)),
    })
}

pub fn error_handle_of_web3(error: web3::Error) ->Json<ApiResponse<String>>{
    println!("{:?}", error);
    Json(ApiResponse {
        success: false,
        code: 500,
        json: None,
        message: Some(format!("{:?}", error)),
    })
}