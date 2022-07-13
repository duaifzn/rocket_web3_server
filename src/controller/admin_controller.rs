use std::str::FromStr;
use crate::dto::request_dto::{MoneyTransferDto};
use crate::dto::response_dto::{
    ApiResponse, TxAddressDto,
};
use crate::middleware::admin_auth_guard::Token;
use crate::util::error_handle::{
    error_handle_of_reqwest, error_handle_of_web3,
};
use crate::util::eth_node::EthNode;
use crate::util::vault::Vault;
use rocket::http::Status;
use rocket::serde::json::Json;
use rocket::State;
use rocket_okapi::openapi;
use web3::types::{Address};

/// # Admin transfer money to one account
///
/// admin transfer money to one account.
#[openapi]
#[post("/admin/money/transfer", format = "json", data = "<body>")]
pub async fn money_transfer(
    _token: Token<'_>,
    vault: &State<Vault>,
    eth_node: &State<EthNode>,
    body: Json<MoneyTransferDto>,
) -> Result<Json<ApiResponse<TxAddressDto>>, (Status, Json<ApiResponse<String>>)> {
    let res = vault
        .get_one_account(&body.to_account_name)
        .await
        .map_err(error_handle_of_reqwest)?;
    let to_address =
        Address::from_str(&res.data.address.replace("0x", "")).unwrap();
    let tx_address = eth_node
        .transfer_1000eth_to_account(to_address)
        .await
        .map_err(error_handle_of_web3)?;

    Ok(Json(ApiResponse {
        success: true,
        code: 200,
        json: Some(TxAddressDto {
            tx_address: format!("{:?}", tx_address),
        }),
        message: None,
    }))
}
