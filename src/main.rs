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

#[launch]
async fn rocket() -> _{
    rocket::build()
        .mount("/api", openapi_get_routes![
            controller::user_controller::index,
            controller::user_controller::signup_one_user,
            controller::user_controller::signin_one_user,
            controller::user_controller::sha256_hash,
            controller::user_controller::send_hash,
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
}
