#[macro_use]
extern crate rocket;
#[macro_use]
extern crate lazy_static;
use rocket_okapi::openapi_get_routes;
use rocket_okapi::swagger_ui::{make_swagger_ui, SwaggerUIConfig};

mod config;
mod contract_interface;
mod controller;
mod database;
mod dto;
mod middleware;
mod model;
mod service;
mod util;
#[cfg(test)]
mod test;

use crate::config::Config;
lazy_static! {
    static ref CONFIG: Config<'static> = Config::load();
}
#[launch]
async fn rocket() -> _ {
    rocket::build()
        .attach(middleware::cors::Cors)
        .mount(
            "/api",
            openapi_get_routes![
                controller::user_controller::signup_one_user,
                controller::user_controller::signin_one_user,
                controller::user_controller::get_account_balance,
                controller::user_controller::cors_handler,
                controller::contract_controller::notarize_hash,
                controller::contract_controller::get_hash,
                controller::contract_controller::deploy_contract,
                controller::contract_controller::get_one_transaction_log,
                controller::contract_controller::get_blockhash_transactions_log,
                controller::contract_controller::get_all_log_of_proof_of_existence,
                controller::contract_controller::get_proof_created_log_of_proof_of_existence,
                controller::admin_controller::money_transfer,
            ],
        )
        .mount(
            "/swagger-ui/",
            make_swagger_ui(&SwaggerUIConfig {
                url: "../api/openapi.json".to_owned(),
                ..Default::default()
            }),
        )
        .manage(database::Mongo::connect().await)
        .manage(util::eth_node::EthNode::connect())
        .manage(util::vault::Vault::new(
            CONFIG.vault_host,
            CONFIG.vault_token,
        ))
}
