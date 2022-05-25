use crate::config::Config;

pub fn local() ->Config<'static>{
    Config { 
        jwt_secret: "secret",
        jwt_expire: 24*60*60,
        mongodb_username: "user",
        mongodb_password: "pass",
        mongodb_address: "localhost:27023",
        mongodb_database_name: "dev",
        vault_host: "localhost:30305",
        vault_token: "s.9ajvNBkgKrdOOi4g08tSuLTR",
    }
}