use crate::config::Config;
use std::borrow::Cow;

pub fn local() -> Config<'static> {
    Config {
        jwt_secret: "secret",
        jwt_expire: 24 * 60 * 60,
        mongodb_username: "user",
        mongodb_password: "pass",
        mongodb_address: "localhost:27023",
        mongodb_database_name: "dev",
        vault_host: Cow::Borrowed("localhost:30305"),
        vault_token: Cow::Borrowed("s.xkWVuDteFqFTFjhOvcQp1oFR"),
        eth_node_host: Cow::Borrowed("20.231.59.102:8545"),
        chain_id: 86530,
        gas_limit: 1071003,
        gas_price: 7,
        miner_private_key: Cow::Borrowed(
            "0x0dc64def930e44654b8661bc054f1ac216baf1d3ae46f40d011caa379ee9d129",
        ),
    }
}
