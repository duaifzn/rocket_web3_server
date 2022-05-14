use crate::database::Mongo;
use crate::model::contract_model::ContractSchema;
use chrono::Local;
use mongodb::bson::doc;
use mongodb::results::InsertOneResult;
use rocket::State;
use web3::types::{H160, H256};
use std::result::Result;

pub async fn insert_one_contract(
    db: &State<Mongo>,
    contract_name: &str,
    contract_address: H160,
    tx_address: H256,
) -> Result<InsertOneResult, String> {
    // let old_nonce = find_one_contract(db, contract_name).await?;
    // match old_nonce {
    //     Some(_) => return Err("contract_name duplicate!".to_string()),
    //     None => {}
    // }
    let data = db
        .Contract
        .insert_one(
            ContractSchema {
                id: None,
                name: contract_name.to_string(),
                address: Some(format!("{:?}", contract_address)),
                tx_address: Some(format!("{:?}", tx_address)),
                create_at: Some(Local::now()),
                update_at: Some(Local::now()),
            },
            None,
        )
        .await;
    match data {
        Ok(nonce) => Ok(nonce),
        Err(err) => Err(err.to_string()),
    }
}

pub async fn find_one_contract(
    db: &State<Mongo>,
    contract_name: &str,
) -> Result<Option<ContractSchema>, String> {
    let data = db
        .Contract
        .find_one(
            doc! {
                "name": contract_name.to_string()
            },
            None,
        )
        .await;
    match data {
        Ok(nonce) => match nonce {
            Some(n) => return Ok(Some(n)),
            None => return Ok(None),
        },
        Err(err) => Err(err.to_string()),
    }
}
