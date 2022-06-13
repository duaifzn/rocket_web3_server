use crate::dto::response_dto::ApiResponse;
use reqwest;
use rocket::serde::json::Json;
use web3;

pub fn error_handle_of_string(error: String) -> Json<ApiResponse<String>> {
    println!("{:?}", error);
    Json(ApiResponse {
        success: false,
        code: 500,
        json: None,
        message: Some(error),
    })
}

pub fn error_handle_of_reqwest(error: reqwest::Error) -> Json<ApiResponse<String>> {
    println!("{:?}", error);
    Json(ApiResponse {
        success: false,
        code: 500,
        json: None,
        message: Some(format!("{:?}", error)),
    })
}

pub fn error_handle_of_web3(error: web3::Error) -> Json<ApiResponse<String>> {
    println!("{:?}", error);
    match error.clone() {
        web3::Error::Rpc(err) => match err.data.clone().take() {
            Some(e) => match e {
                schemars::_serde_json::Value::String(a) => {
                    match hex::decode(&a.replace("0x", "")[136..]) {
                        Ok(v) => match std::str::from_utf8(&v) {
                            Ok(s) => {
                                return Json(ApiResponse {
                                    success: false,
                                    code: 500,
                                    json: None,
                                    message: Some(format!(
                                        "{}: {}",
                                        err.message,
                                        s.trim_matches(char::from(0))
                                    )),
                                })
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
    Json(ApiResponse {
        success: false,
        code: 500,
        json: None,
        message: Some(format!("{:?}", error)),
    })
}
