use crate::config::Config;

pub fn dev() ->Config<'static>{
    Config { 
        jwt_secret: "secret",
        jwt_expire: 24*60*60,
        mongodb_username: "root",
        mongodb_password: "mongopassword",
        mongodb_address: "localhost:27023",
        mongodb_database_name: "herodotus-dev", 
    }
}