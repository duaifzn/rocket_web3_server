#[macro_use] extern crate rocket;
#[macro_use] extern crate lazy_static;
use rocket_okapi::openapi_get_routes;
use rocket_okapi::swagger_ui::{make_swagger_ui, SwaggerUIConfig};

mod controller;
mod model;
mod dto;
mod database;
mod service;
mod util;
mod config;
mod middleware;
mod contract_interface;
#[cfg(test)]
mod test;

use crate::config::Config;
lazy_static!{
    static ref CONFIG: Config<'static> = Config::load();
}
#[launch]
async fn rocket() -> _{
    rocket::build()
        .mount("/api", openapi_get_routes![
            controller::user_controller::index,
            controller::user_controller::signup_one_user,
            controller::user_controller::signin_one_user,
            controller::contract_controller::sha256_hash,
            controller::contract_controller::send_hash,
            controller::contract_controller::is_issuer,
        ])
        .mount(
            "/swagger-ui/",
            make_swagger_ui(&SwaggerUIConfig {
                url: "../api/openapi.json".to_owned(),
                ..Default::default()
            }),
        )
        .manage(database::Mongo::connect().await)
        .manage(util::eth_node::EthNode::connect())
        .manage(util::vault::Vault::new(CONFIG.vault_host, CONFIG.vault_token))
}
