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
        eth_node_host: "52.179.136.216:8545",
        chain_id: 1337,
        gas_limit: 1071003,
        gas_price: 7,
        miner_private_key: "0x7d1fe7133ae962a50f860468ea1351f83e417dbdaab635294efdd8e6e3eef031",
    }
}