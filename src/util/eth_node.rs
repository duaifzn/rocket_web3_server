use std::path::Path;
use std::str::FromStr;
use web3::types::{H160, Address, U256, BlockId, BlockNumber, TransactionRequest, Bytes, H256, TransactionId, Transaction};
use web3::{self, Web3, Result};
use web3::contract::{Contract, Options};
use web3::transports::Http;
use eth_keystore::decrypt_key;
use web3::Error::Decoder;
use hex::FromHex;
use crate::dto::raw_transaction::RawTransaction;
pub struct EthNode{
    pub node_url: String,
    pub web3: Web3<Http>
}

impl EthNode{
    pub fn connect() ->Self{
        let transport = Http::new("http://52.179.136.216:8545").unwrap();
        let web3 = Web3::new(transport);
        
        Self{
            node_url: "http://52.179.136.216:8545".to_string(),
            web3: web3
        }
    }
    pub fn hex_str_to_bytes20(param: &str) ->web3::Result<[u8; 20]>{
        let temp = Vec::from_hex(param).unwrap();
        if temp.len() > 20{
            return Err(Decoder("hex string lengh > 20 after convert!".to_string()));
        }
        let mut bytes20 = [0u8; 20];
        bytes20[..temp.len()].copy_from_slice(&temp);
        Ok(bytes20)
    }
    pub fn hex_str_to_bytes32(param: &str) ->web3::Result<[u8; 32]>{
        let temp = Vec::from_hex(param).unwrap();
        if temp.len() > 32{
            return Err(Decoder("hex string lengh > 32 after convert!".to_string()));
        }
        let mut bytes32 = [0u8; 32];
        bytes32[..temp.len()].copy_from_slice(&temp);
        Ok(bytes32)
    }
    pub async fn get_accounts(&self) ->Result<Vec<H160>>{
        let accounts = self.web3.eth().accounts().await?;
        Ok(accounts)
    }
    pub async fn get_account_balance(&self, account:Address) ->Result<U256>{
        let balance = self.web3.eth().balance(account, None).await?;
        Ok(balance)
    }
    pub async fn create_one_account(&self, password: &str) ->Result<H160>{
        let new_account = self.web3.personal().new_account(password).await?;
        Ok(new_account)
    }
    pub fn keystore_to_private_key(keystore_file: &str, password: &str) ->String{
        let keypath = Path::new(keystore_file);
        let private_key = decrypt_key(&keypath, password).unwrap();
        format!("0x{}", hex::encode(private_key))
    }
    pub async fn get_transaction_nonce(&self, address: &str) ->Result<U256>{
        let new_address = Self::hex_str_to_bytes20(&address.replace("0x", "")).unwrap();
        let nonce_pending = self.web3.eth().transaction_count(
            H160::from(new_address),
            Some(BlockNumber::Pending)).await?;
        let nonce_latest = self.web3.eth().transaction_count(
            H160::from(new_address),
            Some(BlockNumber::Latest)).await?;
        Ok(nonce_pending+nonce_latest)
    }
    pub async fn get_transaction(&self, tx_address: &str) ->Result<Option<Transaction>>{
        let temp = Self::hex_str_to_bytes32(tx_address).unwrap();
        let address = TransactionId::Hash(H256::from(temp));
        let result = self.web3.eth().transaction(address).await?;
        Ok(result)
    }
    pub async fn send_raw_transaction(&self, tx: Bytes) ->Result<H256>{
        let result = self.web3.eth().send_raw_transaction(tx).await?;
        Ok(result)
    }
    pub async fn connect_contract_of_proof_of_existence(&self, address: &str) ->Contract<Http>{
        let contract_address = Address::from_str(address).unwrap();
        let contract = Contract::from_json(
            self.web3.eth(),
            contract_address,
            include_bytes!("../../contract/abi.json")
        ).unwrap();
        contract
    }
    pub fn connect_contract_of_proof_of_existence_by_vault(&self, address: &str) ->Contract<Http>{
        let contract_address = Address::from_str(address).unwrap();
        let contract = Contract::from_json(
            self.web3.eth(),
            contract_address,
            include_bytes!("../../contract/abi.json")
        ).unwrap();
        contract
    }
}