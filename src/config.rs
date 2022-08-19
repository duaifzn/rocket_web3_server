pub mod config_dev;
pub mod config_local;
pub mod config_prod;
use crate::config::config_dev::dev;
use crate::config::config_local::local;
use crate::config::config_prod::prod;
use std::borrow::Cow;
use std::{env, fmt};

#[derive(Debug, Clone, Copy)]
pub enum ConfigEnv {
    #[allow(non_camel_case_types)]
    VAULT_HOST,
    #[allow(non_camel_case_types)]
    VAULT_TOKEN,
    #[allow(non_camel_case_types)]
    ETH_NODE_HOST,
    #[allow(non_camel_case_types)]
    CHAIN_ID,
    #[allow(non_camel_case_types)]
    GAS_LIMIT,
    #[allow(non_camel_case_types)]
    GAS_PRICE,
    #[allow(non_camel_case_types)]
    MINER_PRIVATE_KEY,
}

impl fmt::Display for ConfigEnv {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ConfigEnv::VAULT_HOST => write!(f, "VAULT_HOST"),
            ConfigEnv::VAULT_TOKEN => write!(f, "VAULT_TOKEN"),
            ConfigEnv::ETH_NODE_HOST => write!(f, "ETH_NODE_HOST"),
            ConfigEnv::CHAIN_ID => write!(f, "CHAIN_ID"),
            ConfigEnv::GAS_LIMIT => write!(f, "GAS_LIMIT"),
            ConfigEnv::GAS_PRICE => write!(f, "GAS_PRICE"),
            ConfigEnv::MINER_PRIVATE_KEY => write!(f, "MINER_PRIVATE_KEY"),
        }
    }
}
pub struct Config<'a> {
    pub jwt_secret: &'a str,
    pub jwt_expire: i64,
    pub mongodb_username: &'a str,
    pub mongodb_password: &'a str,
    pub mongodb_address: &'a str,
    pub mongodb_database_name: &'a str,
    pub vault_host: Cow<'a ,str>,
    pub vault_token: Cow<'a ,str>,
    pub eth_node_host: Cow<'a ,str>,
    pub chain_id: u128,
    pub gas_limit: u128,
    pub gas_price: u128,
    pub miner_private_key: Cow<'a ,str>,
}

impl Config<'static> {
    pub fn load() -> Self {
        let args: Vec<String> = env::args().collect();
        match args.len() {
            0 | 1 => return prod(),
            _ => {}
        }
        let env = args[1].as_str();
        match env {
            "local" => return local(),
            "dev" => return dev(),
            "prod" => return prod(),
            _ => return prod(),
        }
    }
    pub fn load_env(key: ConfigEnv) -> Cow<'static, str>{
        match env::var(key.to_string().as_str()) {
            Ok(value) => return Cow::Owned(value),
            Err(err) => {
                println!("load env error: {:?}", err);
                return Cow::Borrowed("")
            },
        }
    }
}