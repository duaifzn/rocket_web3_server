use std::str::FromStr;
use std::string::FromUtf8Error;
use hex::FromHex;
use secp256k1::SecretKey;
use web3::contract::{Contract, Options, Result};
use web3::ethabi::{Token, FixedBytes};
use web3::transports::Http;
use web3::types::{H160, H256};
use web3::signing::SecretKeyRef;
use web3::Error::Decoder;

use crate::dto::raw_transaction::RawTransaction;
pub struct ProofOfExistence{
    pub contract: Contract<Http>,
}

impl ProofOfExistence{
    pub fn hex_str_to_bytes32(param: &str) ->web3::Result<Vec<u8>>{
        let temp = Vec::from_hex(param).unwrap();
        if temp.len() > 32{
            return Err(Decoder("hex string lengh > 32 after convert!".to_string()));
        }
        let mut bytes32 = [0u8; 32];
        bytes32[..temp.len()].copy_from_slice(&temp);
        Ok(bytes32.to_vec())
    }
    pub fn bytes32_to_string(bytes32: Vec<u8>) ->web3::Result<String>{
        if bytes32.len() > 32{
            return Err(Decoder("bytes lengh > 32!".to_string()));
        }
        let temp = String::from_utf8(bytes32);
        let string = match temp{
            Ok(result) => result,
            Err(err) => return Err(Decoder(err.to_string()))
        };
        Ok(string.trim_matches(char::from(0)).to_string())
    }
    pub async fn notarize_hash(&self, private_key: &str,  key: &str, value: &str) ->web3::Result<H256>{
        let key_bytes32 = Token::FixedBytes(ProofOfExistence::hex_str_to_bytes32(key)?);
        let value_bytes32 = Token::FixedBytes(ProofOfExistence::hex_str_to_bytes32(value)?);
        let secret_key = SecretKey::from_str(&private_key.replace("0x", "")).unwrap();
        let result = self.contract.signed_call(
            "notarizeHash",
            (key_bytes32, value_bytes32),
            Options::default(),
            SecretKeyRef::new(&secret_key)).await;
        result
    }
    pub async fn get_hash(&self, accuont: H160,  key: &str) ->Result<FixedBytes>{
        let key_bytes32 = Token::FixedBytes(ProofOfExistence::hex_str_to_bytes32(key)?);
        let result = self.contract.query::<FixedBytes, _, _, _>(
            "getHash",
            key_bytes32,
            accuont,
            Options::default(),
            None).await;
        result
    }
    pub async fn revoke_hash(&self, private_key: &str, key: &str) ->web3::Result<H256>{
        let key_bytes32 = Token::FixedBytes(ProofOfExistence::hex_str_to_bytes32(key)?);
        let secret_key = SecretKey::from_str(&private_key.replace("0x", "")).unwrap();
        let result = self.contract.signed_call(
            "revokeHash",
            key_bytes32,
            Options::default(),
            SecretKeyRef::new(&secret_key)).await;
        result
    }
    pub async fn is_revoked(&self, accuont: H160,  key: &str) ->Result<bool>{
        let key_bytes32 = Token::FixedBytes(ProofOfExistence::hex_str_to_bytes32(key)?);
        let result = self.contract.query::<bool, _, _, _>(
            "getHash",
            key_bytes32,
            accuont,
            Options::default(),
            None).await;
        result
    }
    pub async fn add_issuer(&self, private_key: &str, issuer: H160) ->web3::Result<H256>{
        let issuer_bytes32 = Token::Address(issuer);
        let secret_key = SecretKey::from_str(&private_key.replace("0x", "")).unwrap();
        let result = self.contract.signed_call(
            "addIssuer",
            issuer_bytes32,
            Options::default(),
            SecretKeyRef::new(&secret_key)).await;
        result
    }
    pub async fn del_issuer(&self, private_key: &str, issuer: H160) ->web3::Result<H256>{
        let issuer_bytes32 = Token::Address(issuer);
        let secret_key = SecretKey::from_str(&private_key.replace("0x", "")).unwrap();
        let result = self.contract.signed_call(
            "delIssuer",
            issuer_bytes32,
            Options::default(),
            SecretKeyRef::new(&secret_key)).await;
        result
    }
    pub async fn is_issuer(&self, account: H160) -> Result<bool>{
        let result = self.contract.query::<bool, _, _, _>(
            "isIssuer",
            account,
            account,
            Options::default(),
            None).await;
        result
    }
    pub async fn transfer_ownership(&self, private_key: &str, owner: H160) ->web3::Result<H256>{
        let owner_bytes32 = Token::Address(owner);
        let secret_key = SecretKey::from_str(&private_key.replace("0x", "")).unwrap();
        let result = self.contract.signed_call(
            "transferOwnership",
            owner_bytes32,
            Options::default(),
            SecretKeyRef::new(&secret_key)).await;
        result
    }
}