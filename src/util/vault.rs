use crate::dto::raw_transaction::RawTransaction;
use crate::dto::response_dto::{VaultAccountDto, VaultSignDto};
use reqwest;
use reqwest::Result;
use sha2::{Digest, Sha256};
use std::collections::HashMap;
use crate::config::Config;
lazy_static!{
    static ref CONFIG: Config<'static> = Config::load();
}
pub struct Vault {
    pub host: String,
    pub token: String,
}

impl Vault {
    pub fn new(host: &str, token: &str) -> Self {
        Self {
            host: host.to_string(),
            token: token.to_string(),
        }
    }
    pub fn sha256_hash(data: &str) -> String {
        let mut hasher = Sha256::new();
        hasher.update(data.to_string().as_bytes());
        let done = hasher.finalize();
        let hash_data = format!("{:X}", done);
        hash_data
    }

    pub async fn unseal_by_keys(&self, keys: Vec<&str>) -> Result<()> {
        for key in keys.into_iter() {
            let mut map = HashMap::new();
            map.insert("key", key.to_string());
            let client = reqwest::Client::new();
            client
                .put(format!("http://{}/v1/sys/unseal", self.host))
                .json(&map)
                .send()
                .await?;
        }
        Ok(())
    }
    pub async fn create_one_raw_transaction(
        &self,
        address_to: &str,
        nonce: u64,
        data: Vec<u8>,
    ) -> RawTransaction {
        RawTransaction {
            address_to: address_to.to_string(),
            data: hex::encode(data),
            encoding: "hex".to_string(),
            amount: "0".to_string(),
            nonce: nonce.to_string(),
            gas_limit: CONFIG.gas_limit.to_string(),
            gas_price: CONFIG.gas_price.to_string(),
            chainID: CONFIG.chain_id.to_string(),
        }
    }
    pub async fn create_one_account(&self, account_name: &str) -> Result<VaultAccountDto> {
        let hash_account = Self::sha256_hash(account_name);
        let client = reqwest::Client::new();
        let res: VaultAccountDto = client
            .post(format!(
                "http://{}/v1/ethplugin/accounts/{}",
                self.host, hash_account
            ))
            .header("Authorization", format!("Bearer {}", self.token))
            .send()
            .await?
            .json()
            .await?;
        Ok(res)
    }
    pub async fn get_one_account(&self, account_name: &str) -> Result<VaultAccountDto> {
        let hash_account = Self::sha256_hash(account_name);
        let client = reqwest::Client::new();
        let res: VaultAccountDto = client
            .get(format!(
                "http://{}/v1/ethplugin/accounts/{}",
                self.host, hash_account
            ))
            .header("Authorization", format!("Bearer {}", self.token))
            .send()
            .await?
            .json()
            .await?;
        Ok(res)
    }
    pub async fn sign_one_transaction(
        &self,
        account_name: &str,
        raw_transaction: RawTransaction,
    ) -> Result<VaultSignDto> {
        let hash_account = Self::sha256_hash(account_name);
        let client = reqwest::Client::new();
        let res: VaultSignDto = client
            .post(format!(
                "http://{}/v1/ethplugin/accounts/{}/sign-tx",
                self.host, hash_account
            ))
            .header("Authorization", format!("Bearer {}", self.token))
            .json(&raw_transaction)
            .send()
            .await?
            .json()
            .await?;
        Ok(res)
    }
}
