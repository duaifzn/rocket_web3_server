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
        eth_node_host: "211.73.81.45:8545",
        chain_id: 86532,
        gas_limit: 1071003,
        gas_price: 25000,
        miner_private_key: "0x08a1aa1bef5948f97454d6ca9c4b96c07b23a666267b3c5457040510ac19cdb0",
    }
}