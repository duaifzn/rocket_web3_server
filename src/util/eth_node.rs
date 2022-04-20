use std::str::FromStr;
use web3::types::{H160, Address, U256, BlockId};
use web3::{self, Web3, Result};
use web3::contract::{Contract, Options};
use web3::transports::Http;

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
    pub async fn get_accounts(&self) ->Result<Vec<H160>>{
        let accounts = self.web3.eth().accounts().await?;
        Ok(accounts)
    }
    pub async fn get_account_balance(&self, account:Address) ->Result<U256>{
        let balance = self.web3.eth().balance(account, None).await?;
        Ok(balance)
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
}