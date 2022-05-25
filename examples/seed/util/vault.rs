use crate::dto::response_dto::{VaultAccountDto};
use reqwest;
use reqwest::{Result};
use sha2::{Digest, Sha256};
use std::collections::HashMap;

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
}
