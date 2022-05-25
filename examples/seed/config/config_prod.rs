use crate::config::Config;

pub fn prod() ->Config<'static>{
    Config { 
        jwt_secret: "secret",
        jwt_expire: 24*60*60,
        mongodb_username: "user",
        mongodb_password: "pass",
        mongodb_address: "20.110.233.35:8001,20.110.233.35:8002,20.110.233.35:8003",
        mongodb_database_name: "dev",
        vault_host: "localhost:30305",
        vault_token: "hvs.dU9cdQ8Txg6FpRfpKjSi1KlW",
    }
}