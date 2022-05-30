use std::str::FromStr;
use hex::FromHex;
use secp256k1::SecretKey;
use web3::contract::{Contract, Options, Result};
use web3::ethabi::{Token, FixedBytes, Log};
use web3::transports::Http;
use web3::types::{H160, H256};
use web3::signing::SecretKeyRef;
use web3::Error::Decoder;
use crate::dto::response_dto::DecodeEventLogDto;
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
        let string = hex::encode(bytes32);
        Ok(string)
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
    pub fn decode_event_log(event_name: &str, log: Option<Log>) ->DecodeEventLogDto{
        match log{
            Some(_) => {},
            None => return DecodeEventLogDto{
                event_name: None,
                previous_owner: None,
                new_owner: None,
                key: None,
                value: None,
                issuer_account: None,
            }
        }
        match event_name {
            "ProofCreated" => {
                let log = log.unwrap();
                let key = hex::encode(log.params[0].value.clone().into_fixed_bytes().unwrap());
                let value = hex::encode(log.params[1].value.clone().into_fixed_bytes().unwrap());
                return DecodeEventLogDto{
                    event_name: Some("ProofCreated".to_string()),
                    previous_owner: None,
                    new_owner: None,
                    key: Some(key),
                    value: Some(value),
                    issuer_account: None,
                }
            },
            "OwnershipTransferred" => {
                let log = log.unwrap();
                let previous_owner = hex::encode(log.params[0].value.clone().into_address().unwrap());
                let new_owner = hex::encode(log.params[1].value.clone().into_address().unwrap());
                return DecodeEventLogDto{
                    event_name: Some("OwnershipTransferred".to_string()),
                    previous_owner: Some(previous_owner),
                    new_owner: Some(new_owner),
                    key: None,
                    value: None,
                    issuer_account: None,
                }
            },
            "IssuerAdded" => {
                let log = log.unwrap();
                let account = hex::encode(log.params[0].value.clone().into_address().unwrap());
                return DecodeEventLogDto{
                    event_name: Some("IssuerAdded".to_string()),
                    previous_owner: None,
                    new_owner: None,
                    key: None,
                    value: None,
                    issuer_account: Some(account),
                }
            },
            "IssuerRemoved" => {
                let log = log.unwrap();
                let account = hex::encode(log.params[0].value.clone().into_address().unwrap());
                return DecodeEventLogDto{
                    event_name: Some("IssuerRemoved".to_string()),
                    previous_owner: None,
                    new_owner: None,
                    key: None,
                    value: None,
                    issuer_account: Some(account),
                }
            },
            "IssuerChecked" => {
                let log = log.unwrap();
                let account = hex::encode(log.params[0].value.clone().into_address().unwrap());
                return DecodeEventLogDto{
                    event_name: Some("IssuerChecked".to_string()),
                    previous_owner: None,
                    new_owner: None,
                    key: None,
                    value: None,
                    issuer_account: Some(account),
                }
            },
            "ProofRevoked" => {
                let log = log.unwrap();
                let key = hex::encode(log.params[0].value.clone().into_fixed_bytes().unwrap());
                return DecodeEventLogDto{
                    event_name: Some("ProofRevoked".to_string()),
                    previous_owner: None,
                    new_owner: None,
                    key: Some(key),
                    value: None,
                    issuer_account: None,
                }
            },
            _ => return DecodeEventLogDto{
                event_name: None,
                previous_owner: None,
                new_owner: None,
                key: None,
                value: None,
                issuer_account: None,
            }
        };
    }
}