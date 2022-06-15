use crate::dto::response_dto::ApiResponse;
use reqwest;
use rocket::{http::Status, serde::json::Json};
use web3;

pub fn error_handle_of_string(error: String) -> (Status, Json<ApiResponse<String>>) {
    println!("{:?}", error);
    (
        Status::InternalServerError,
        Json(ApiResponse {
            success: false,
            code: 500,
            json: None,
            message: Some(error),
        }),
    )
}

pub fn error_handle_of_reqwest(error: reqwest::Error) -> (Status, Json<ApiResponse<String>>) {
    println!("{:?}", error);
    (
        Status::InternalServerError,
        Json(ApiResponse {
            success: false,
            code: 500,
            json: None,
            message: Some(format!("{:?}", error)),
        }),
    )
}

pub fn error_handle_of_web3(error: web3::Error) -> (Status, Json<ApiResponse<String>>) {
    println!("{:?}", error);
    match error.clone() {
        web3::Error::Rpc(err) => match err.data.clone().take() {
            Some(e) => match e {
                schemars::_serde_json::Value::String(a) => {
                    match hex::decode(&a.replace("0x", "")[136..]) {
                        Ok(v) => match std::str::from_utf8(&v) {
                            Ok(s) => {
                                return (
                                    Status::InternalServerError,
                                    Json(ApiResponse {
                                        success: false,
                                        code: 500,
                                        json: None,
                                        message: Some(format!(
                                            "{}: {}",
                                            err.message,
                                            s.trim_matches(char::from(0))
                                        )),
                                    }),
                                )
                            }
                            Err(_) => {}
                        },
                        Err(_) => {}
                    }
                }
                _ => {}
            },
            None => {}
        },
        _ => {}
    }
    (
        Status::InternalServerError,
        Json(ApiResponse {
            success: false,
            code: 500,
            json: None,
            message: Some(format!("{:?}", error)),
        }),
    )
}
