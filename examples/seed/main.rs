#[macro_use] extern crate lazy_static;

mod config;
mod database;
mod model;
mod util;
mod dto;
use crate::{database::Mongo, config::Config};
use crate::model::user_model::UserSchema;
use bcrypt::{hash, DEFAULT_COST};
use chrono::Local;
use util::vault::Vault;
lazy_static! {
    static ref CONFIG: Config<'static> = Config::load();
}
enum Role{
    Admin,
    User,
    Visitor,
}

#[rocket::main]
async fn main() {
    println!("####### mongo seed start #######");
    let db = Mongo::connect().await;
    let seed = vec![
        UserSchema{
            id: None,
            email: "admin123@gmail.com".to_owned(),
            password: hash("admin123".to_owned(), DEFAULT_COST).unwrap(),
            role: Role::Admin as u8,
            create_at: Some(Local::now()),
            update_at: Some(Local::now()),
        },
    ];

    let _ = db.User.insert_many(seed, None).await;
    println!("####### mongo seed complete #######");

    println!("####### vault seed start #######");
    let vault = Vault::new(CONFIG.vault_host, CONFIG.vault_token);
    let a = vault.create_one_account("admin123@gmail.com").await;
    match a{
        Ok(_) => {},
        Err(err) => println!("{:?}", err)
    }
    println!("####### vault seed complete #######");
}