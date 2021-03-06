use chrono::Local;
use mongodb::bson::doc;
use mongodb::error::{Result, Error};
use mongodb::results::InsertOneResult;
use rocket::State;
use rocket::serde::json::Json;
use bcrypt::{hash, verify, DEFAULT_COST};
use crate::database::Mongo;
use crate::dto::request_dto::UserDto;
use crate::model::user_model::UserSchema;
use crate::dto::role::Role;

pub async fn insert_one_user(db: &State<Mongo>, user: Json<UserDto>) ->Result<InsertOneResult>{
    let hashed = hash(user.password.to_owned(), DEFAULT_COST).unwrap();
    let new_user = UserSchema{
        id: None,
        email: user.email.to_owned(),
        password: hashed,
        role: Role::User as u8,
        public_key: None,
        create_at: Some(Local::now()),
        update_at: Some(Local::now()),
    }; 
    let data = db.User.insert_one(new_user, None).await?;
    Ok(data)
}

pub async fn verify_one_user(db: &State<Mongo>, user: Json<UserDto>) ->Result<(bool, Option<UserSchema>)>{
    let old_user = db.User.find_one(doc!{
        "email": user.email.to_owned()
    }, None).await?;
    match old_user{
        Some(result) => Ok((verify(user.password.to_owned(), &result.password).unwrap(), Some(result))),
        None => Ok((false, None))
    }
}

pub async fn find_one_user(db: &State<Mongo>, user: &Json<UserDto>) ->Result<bool>{
    let old_user = db.User.find_one(doc!{
        "email": user.email.to_owned()
    }, None).await?;
    match old_user{
        Some(result) => Ok(true),
        None => Ok(false)
    }
}

pub async fn find_one_user_public_key(db: &State<Mongo>, email: String) ->Result<Option<String>>{
    let user = db.User.find_one(doc!{
        "email": email.to_owned()
    }, None).await?;
    match user{
        Some(result) => {
            match result.public_key{
                Some(key) => return Ok(Some(key)),
                None => return Ok(None),
            }
        },
        None => Ok(None)
    }
}