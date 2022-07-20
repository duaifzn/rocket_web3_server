use crate::config::Config;

pub fn prod() ->Config<'static>{
    Config { 
        jwt_secret: "secret",
        jwt_expire: 24*60*60,
        mongodb_username: "user",
        mongodb_password: "pass",
        mongodb_address: "web3-mongo:27017",
        mongodb_database_name: "web3",
        vault_host: "10.26.3.165:30305",
        vault_token: "s.qgNdGJiWrNf2my3n6AXaNAQY",
    }
}