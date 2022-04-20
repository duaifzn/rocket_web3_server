use crate::dto::request_dto::{UserDto, RawDto};
use crate::dto::response_dto::{
    ApiResponse,
    CreateOneUserDto, SigninOneUserDto, Sha256HashDto
};
use crate::rocket;
use rocket::local::blocking::Client;
use rocket::serde::json;
use serial_test::serial;

#[async_std::test]
#[serial]
async fn post_signup(){
    let server = rocket().await;
    let client = Client::tracked(server).expect("valid rocket instance");
    let res = client.post("/api/signup").json(&UserDto{
        email: "duaifzn12345@gmail.com".to_string(),
        password: "asd123".to_string()
    }).dispatch();
    let res_data = json::from_str::<ApiResponse<CreateOneUserDto>>(
        &res.into_string().unwrap()
    ).unwrap();
    assert_eq!(res_data.success, true);
}

#[async_std::test]
#[serial]
async fn post_signup_duplicate_error(){
    let server = rocket().await;
    let client = Client::tracked(server).expect("valid rocket instance");
    let res = client.post("/api/signup").json(&UserDto{
        email: "duaifzn12345@gmail.com".to_string(),
        password: "asd123".to_string()
    }).dispatch();
    let res_data = json::from_str::<ApiResponse<CreateOneUserDto>>(
        &res.into_string().unwrap()
    ).unwrap();
    assert_eq!(res_data.success, false);
}

#[async_std::test]
#[serial]
async fn post_signin(){
    let server = rocket().await;
    let client = Client::tracked(server).expect("valid rocket instance");
    client.post("/api/signup").json(&UserDto{
        email: "duaifzn123456@gmail.com".to_string(),
        password: "asd123".to_string()
    }).dispatch();
    let res = client.post("/api/signin").json(&UserDto{
        email: "duaifzn123456@gmail.com".to_string(),
        password: "asd123".to_string()
    }).dispatch();
    let res_data = json::from_str::<ApiResponse<SigninOneUserDto>>(
        &res.into_string().unwrap()
    ).unwrap();
    assert_eq!(res_data.success, true);
}

#[async_std::test]
#[serial]
async fn post_signin_password_error(){
    let server = rocket().await;
    let client = Client::tracked(server).expect("valid rocket instance");
    let res = client.post("/api/signin").json(&UserDto{
        email: "duaifzn123456@gmail.com".to_string(),
        password: "asd1236".to_string()
    }).dispatch();
    let res_data = json::from_str::<ApiResponse<SigninOneUserDto>>(
        &res.into_string().unwrap()
    ).unwrap();
    assert_eq!(res_data.success, false);
}

#[async_std::test]
#[serial]
async fn get_hash(){
    let server = rocket().await;
    let client = Client::tracked(server).expect("valid rocket instance");
    let res = client.get("/api/hash").json(&RawDto{
        raw_data: "test12345".to_string()
    }).dispatch();
    let res_data = json::from_str::<ApiResponse<Sha256HashDto>>(
        &res.into_string().unwrap()
    ).unwrap();
    assert_eq!(res_data.success, true);
}