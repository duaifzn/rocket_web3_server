pub mod config_dev;
pub mod config_prod;
use crate::config::config_dev::dev;
use crate::config::config_prod::prod;
use std::env;
pub struct Config<'a>{
    pub jwt_secret: &'a str,
    pub jwt_expire: i64, 
    pub mongodb_username: &'a str,
    pub mongodb_password: &'a str,
    pub mongodb_address: &'a str,
    pub mongodb_database_name: &'a str,
    pub vault_host:  &'a str,
    pub vault_token: &'a str,
    pub private_key: &'a str,
}

impl Config<'static>{
    pub fn load() ->Self{
        let args: Vec<String> = env::args().collect();
        match args.len(){
            0 | 1 => return dev(),
            _ => {},
        }
        let env = args[1].as_str();
        match env{
            "dev" => return dev(),
            "prod" => return prod(),
            _ => return dev(),
        } 
    }
}