use openssl::{
    asn1::Asn1Time,
    hash::MessageDigest,
    nid::Nid,
    pkey::{PKey, Private},
    rsa::{Padding, Rsa},
    x509::{X509Name, X509},
};
use rocket::State;
use std::{fs::File, io::Write};

use crate::{database::Mongo, service::user_service::find_one_user_public_key};
pub struct OpenSSL {
    keypair: PKey<Private>,
}

impl OpenSSL {
    pub fn new_keypair() -> Self {
        let rsa = Rsa::generate(2048).unwrap();
        Self {
            keypair: PKey::from_rsa(rsa).unwrap(),
        }
    }
    pub async fn public_key_decrypt(
        db: &State<Mongo>,
        account_email: String,
        data: String,
    ) -> Result<String, String> {
        let pubkey = find_one_user_public_key(db, account_email).await;
        match pubkey {
            Ok(key) => match key {
                Some(k) => {
                    let public_key = Rsa::public_key_from_pem(k.as_bytes()).unwrap();
                    let mut dec_result = vec![0; public_key.size() as usize];
                    let len = public_key
                        .public_decrypt(&data.as_bytes(), &mut dec_result, Padding::PKCS1)
                        .unwrap();

                    Ok(std::str::from_utf8(&dec_result[..len]).unwrap().to_string())
                }
                None => return Err("public key is none!".to_string()),
            },
            Err(err) => return Err(format!("{:?}", err)),
        }
    }
}
