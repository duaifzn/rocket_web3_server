use crate::config::Config;

pub fn dev() ->Config<'static>{
    Config { 
        jwt_secret: "secret",
        jwt_expire: 24*60*60,
        mongodb_username: "root",
        mongodb_password: "root",
        mongodb_address: "web3-mongo:27017",
        mongodb_database_name: "web3-dev",
        vault_host: "52.179.136.216:30305",
        vault_token: "s.Z8uTnozQWThsJymRqK7Dgbre",
        private_key: "0x7d1fe7133ae962a50f860468ea1351f83e417dbdaab635294efdd8e6e3eef031",
    }
}