use crate::config::Config;

pub fn dev() ->Config<'static>{
    Config { 
        jwt_secret: "secret",
        jwt_expire: 24*60*60,
        mongodb_username: "user",
        mongodb_password: "pass",
        mongodb_address: "web3-mongo:27017",
        mongodb_database_name: "web3-dev",
        vault_host: "20.242.94.251:30305",
        vault_token: "s.d9Ro8vMal7TDv53RrymCt0ZM",
        eth_node_host: "20.231.59.102:8545",
        chain_id: 86530,
        gas_limit: 1071003,
        gas_price: 7,
        miner_private_key: "0x0dc64def930e44654b8661bc054f1ac216baf1d3ae46f40d011caa379ee9d129",
    }
}