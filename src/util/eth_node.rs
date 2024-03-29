use crate::config::Config;
use eth_keystore::decrypt_key;
use hex::FromHex;
use rocket::futures::future::join_all;
use rocket::tokio::{self, sync::RwLock};
use rocket::tokio::time::{sleep, Duration};
use secp256k1::SecretKey;
use sha2::{Digest, Sha256};
use std::path::Path;
use std::str::FromStr;
use std::sync::Arc;
use std::net::TcpStream;
use web3::contract::Contract;
use web3::ethabi::{Events, Log, RawLog};
use web3::transports::Http;
use web3::types::{
    Address, BlockId, BlockNumber, Bytes, FilterBuilder, Transaction, TransactionId,
    TransactionParameters, TransactionReceipt, H160, H256, U256,
};
use web3::Error::{Decoder, InvalidResponse};
use web3::{self, Result, Web3};
lazy_static! {
    static ref CONFIG: Config<'static> = Config::load();
}
#[derive(Debug)]
pub struct EthNode {
    pub node_url: Arc<RwLock<String>>,
    pub web3: Arc<RwLock<Web3<Http>>>,
}

impl EthNode {
    pub fn connect() -> Self {
        let node_url_string: Vec<&str> = CONFIG.eth_node_host.split(',').collect();
        let node_url = format!("http://{}", node_url_string[0]);
        let transport = Http::new(&node_url).unwrap();
        let web3 = Web3::new(transport);

        Self {
            node_url: Arc::new(RwLock::new(node_url)),
            web3: Arc::new(RwLock::new(web3)),
        }
    }
    pub fn check_eth_node_health(&self) {
        let node_url_clone = Arc::clone(&self.node_url);
        let web3_clone = Arc::clone(&self.web3);
        tokio::spawn(async move {
            loop {
                tokio::time::sleep(tokio::time::Duration::from_millis(5000)).await;
                let node_urls: Vec<&str> = CONFIG.eth_node_host.split(',').collect();
                let first_node_url = node_urls[0];
                let node_url_temp = node_urls.into_iter().find(|node_url| {
                    let connect_test = TcpStream::connect(node_url);
                    match connect_test {
                        Ok(_) => return true,
                        Err(_) => return false,
                    }
                });
                let node_url = match node_url_temp {
                    Some(url) => url,
                    None => {
                        println!(
                            "{:?}",
                            web3::error::Error::InvalidResponse(
                                "All node urls is invalid".to_string()
                            )
                        );
                        return first_node_url.to_string();
                    }
                };
                let mut a = node_url_clone.write().await;
                *a = format!("http://{}", node_url);
                let mut b = web3_clone.write().await;
                *b = Web3::new(Http::new(&format!("http://{}", node_url)).unwrap())
            }
        });
    }
    pub fn hex_str_to_bytes20(param: &str) -> web3::Result<[u8; 20]> {
        let temp = Vec::from_hex(param).unwrap();
        if temp.len() > 20 {
            return Err(Decoder("hex string lengh > 20 after convert!".to_string()));
        }
        let mut bytes20 = [0u8; 20];
        bytes20[..temp.len()].copy_from_slice(&temp);
        Ok(bytes20)
    }
    pub fn hex_str_to_bytes32(param: &str) -> web3::Result<[u8; 32]> {
        let temp = Vec::from_hex(param).unwrap();
        if temp.len() > 32 {
            return Err(Decoder("hex string lengh > 32 after convert!".to_string()));
        }
        let mut bytes32 = [0u8; 32];
        bytes32[..temp.len()].copy_from_slice(&temp);
        Ok(bytes32)
    }
    pub fn sha256_hash(data: &str) -> String {
        let mut hasher = Sha256::new();
        hasher.update(data.to_string().as_bytes());
        let done = hasher.finalize();
        let hash_data = format!("{:X}", done);
        hash_data
    }
    pub async fn get_accounts(&self) -> Result<Vec<H160>> {
        let accounts = self.web3.read().await.eth().accounts().await?;
        Ok(accounts)
    }
    pub async fn get_account_balance(&self, account: Address) -> Result<U256> {
        let balance = self
            .web3
            .read()
            .await
            .eth()
            .balance(account, None)
            .await?;
        Ok(balance)
    }
    pub async fn create_one_account(&self, password: &str) -> Result<H160> {
        let new_account = self
            .web3
            .read()
            .await
            .personal()
            .new_account(password)
            .await?;
        Ok(new_account)
    }
    pub fn keystore_to_private_key(keystore_file: &str, password: &str) -> String {
        let keypath = Path::new(keystore_file);
        let private_key = decrypt_key(&keypath, password).unwrap();
        format!("0x{}", hex::encode(private_key))
    }
    pub async fn get_transaction_count(&self, address: &str) -> Result<U256> {
        let new_address = Self::hex_str_to_bytes20(&address).unwrap();
        let nonce_latest = self
            .web3
            .read()
            .await
            .eth()
            .transaction_count(H160::from(new_address), Some(BlockNumber::Latest))
            .await?;
        Ok(nonce_latest)
    }
    pub async fn get_transaction(&self, tx_address: H256) -> Result<Option<Transaction>> {
        let address = TransactionId::Hash(tx_address);
        let result = self.web3.read().await.eth().transaction(address).await?;
        Ok(result)
    }
    pub async fn send_raw_transaction(&self, tx: Bytes) -> Result<H256> {
        let result = self
            .web3
            .read()
            .await
            .eth()
            .send_raw_transaction(tx)
            .await?;
        Ok(result)
    }
    pub async fn transfer_1000eth_to_account(&self, address: Address) -> Result<H256> {
        let miner_private_key = CONFIG.miner_private_key.to_string();
        let private_key = SecretKey::from_str(&miner_private_key.replace("0x", "")).unwrap();
        let tx_object = TransactionParameters {
            to: Some(address),
            value: U256::exp10(21),
            gas_price: Some(U256::from(25000)),
            ..Default::default()
        };
        let signed = self
            .web3
            .read()
            .await
            .accounts()
            .sign_transaction(tx_object, &private_key)
            .await?;
        let result = self
            .web3
            .read()
            .await
            .eth()
            .send_raw_transaction(signed.raw_transaction)
            .await?;

        Ok(result)
    }
    pub async fn wait_contract_address_of_transaction_receipt(
        &self,
        tx_addres: H256,
    ) -> Result<H160> {
        loop {
            sleep(Duration::from_secs(2)).await;
            let receipt = self
                .web3
                .read()
                .await
                .eth()
                .transaction_receipt(tx_addres)
                .await?;
            match receipt {
                Some(receipt) => match receipt.contract_address {
                    Some(contract_address) => return Ok(contract_address),
                    None => return Err(InvalidResponse("contract_address is None!".to_string())),
                },
                None => {}
            }
        }
    }
    pub async fn get_all_transactions_of_blockhash(
        &self,
        blockhash: H256,
    ) -> Result<(Option<Vec<H256>>, Option<U256>)> {
        let block = self
            .web3
            .read()
            .await
            .eth()
            .block_with_txs(BlockId::Hash(blockhash))
            .await?;
        match block {
            Some(block_msg) => {
                let txs = block_msg.transactions.clone();
                let txs_hash: Vec<H256> = txs.to_vec().into_iter().map(|tx| tx.hash).collect();
                Ok((Some(txs_hash), Some(block_msg.timestamp)))
            }
            None => Ok((None, None)),
        }
    }
    pub async fn get_many_transaction_receipts(
        &self,
        txs: Vec<H256>,
    ) -> Vec<Option<TransactionReceipt>> {
        let process = txs.to_vec().into_iter().map(|tx| async move {
            let result = self
                .web3
                .read()
                .await
                .eth()
                .transaction_receipt(tx)
                .await;
            match result {
                Ok(receipt) => receipt,
                Err(err) => {
                    println!("{:?}", err);
                    None
                }
            }
        });
        join_all(process).await
    }
    pub async fn get_one_transaction_receipt(
        &self,
        tx_address: H256,
    ) -> Result<Option<TransactionReceipt>> {
        let receipt = self
            .web3
            .read()
            .await
            .eth()
            .transaction_receipt(tx_address)
            .await?;
        Ok(receipt)
    }
    pub async fn get_blockhash_timestamp(&self, blockhash: H256) -> Result<Option<U256>> {
        let block = self
            .web3
            .read()
            .await
            .eth()
            .block_with_txs(BlockId::Hash(blockhash))
            .await?;
        match block {
            Some(block_msg) => Ok(Some(block_msg.timestamp)),
            None => Ok(None),
        }
    }
    pub async fn get_blockhash_parent_hash(&self, blockhash: H256) -> Result<Option<H256>> {
        let block = self
            .web3
            .read()
            .await
            .eth()
            .block_with_txs(BlockId::Hash(blockhash))
            .await?;
        match block {
            Some(block_msg) => Ok(Some(block_msg.parent_hash)),
            None => Ok(None),
        }
    }
    pub fn decode_log(events: Events, raw_log: RawLog) -> (Option<String>, Option<Log>) {
        for event in events {
            let result = event.clone().parse_log(raw_log.clone());
            match result {
                Ok(log) => return (Some(event.name.clone()), Some(log)),
                Err(_) => {}
            }
        }
        return (None, None);
    }
    pub async fn connect_contract_of_proof_of_existence(&self, address: &str) -> Contract<Http> {
        let contract_address = Address::from_str(address).unwrap();
        let contract = Contract::from_json(
            self.web3.read().await.eth(),
            contract_address,
            include_bytes!("../../contract/abi.json"),
        )
        .unwrap();
        contract
    }
    pub async fn block_number_range_of_address_log(
        &self,
        address: Address,
        start: u128,
        end: u128,
    ) -> Result<(String, String)> {
        let mut min: u128 = 0;
        let mut max: u128 = 0;
        let filter = FilterBuilder::default()
            .from_block(BlockNumber::Earliest)
            .to_block(BlockNumber::Latest)
            .address(vec![address])
            .build();
        let filter = self
            .web3
            .read()
            .await
            .eth_filter()
            .create_logs_filter(filter)
            .await?;
        let logs = filter.logs().await?;
        for log in logs {
            let time = self
                .get_blockhash_timestamp(log.block_hash.unwrap())
                .await?;
            match time {
                Some(t) => {
                    let a = format!("{:?}", t).parse::<u128>().unwrap();
                    let blocknumber = format!("{:?}", log.block_number.unwrap())
                        .parse::<u128>()
                        .unwrap();
                    if a >= start && a <= end {
                        if min == 0 {
                            min = blocknumber;
                        }
                        if blocknumber < min {
                            min = blocknumber;
                        }
                        if blocknumber > max {
                            max = blocknumber;
                        }
                    }
                }
                None => {}
            }
        }
        Ok((format!("{min:x}"), format!("{max:x}")))
    }
}
