use crate::config::{Config, ConfigEnv};

pub fn prod() -> Config<'static> {
    Config {
        jwt_secret: "secret",
        jwt_expire: 24 * 60 * 60,
        mongodb_username: "user",
        mongodb_password: "pass",
        mongodb_address: "web3-mongo:27017",
        mongodb_database_name: "web3",
        vault_host: Config::load_env(ConfigEnv::VAULT_HOST),
        vault_token: Config::load_env(ConfigEnv::VAULT_TOKEN),
        eth_node_host: Config::load_env(ConfigEnv::ETH_NODE_HOST),
        chain_id: Config::load_env(ConfigEnv::CHAIN_ID)
            .parse::<u128>()
            .unwrap_or(86532),
        gas_limit: Config::load_env(ConfigEnv::GAS_LIMIT)
            .parse::<u128>()
            .unwrap_or(1071003),
        gas_price: Config::load_env(ConfigEnv::GAS_PRICE)
            .parse::<u128>()
            .unwrap_or(25000),
        miner_private_key: Config::load_env(ConfigEnv::MINER_PRIVATE_KEY),
    }
}
