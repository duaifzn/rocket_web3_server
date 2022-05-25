use chrono::{DateTime, Local};
use rocket::serde::{Serialize, Deserialize, Serializer};
use mongodb::bson::oid::ObjectId;


pub fn serialize_object_id<S>(object_id: &Option<ObjectId>, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    match object_id {
      Some(ref object_id) => serializer.serialize_some(object_id.to_string().as_str()),
      None => serializer.serialize_none()
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct UserSchema{
    #[serde(rename(serialize = "_id", deserialize = "id"),
    skip_serializing_if = "Option::is_none",
    serialize_with = "serialize_object_id")]
    pub id: Option<ObjectId>,
    pub email: String,
    pub password: String,
    pub role: u8,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub create_at: Option<DateTime<Local>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub update_at: Option<DateTime<Local>>,
}