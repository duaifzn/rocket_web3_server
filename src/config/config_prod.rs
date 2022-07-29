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
        eth_node_host: "10.26.3.162:8545",
        chain_id: 86532,
        gas_limit: 1071003,
        gas_price: 25000,
        miner_private_key: "0x08a1aa1bef5948f97454d6ca9c4b96c07b23a666267b3c5457040510ac19cdb0",
    }
}