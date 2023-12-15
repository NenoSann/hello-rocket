use mongodb::{
    bson::{
        doc, oid,
        serde_helpers::{
            serialize_bson_datetime_as_rfc3339_string, serialize_hex_string_as_object_id,
        },
        DateTime, Document,
    },
    error::Error,
    options::ClientOptions,
    Client, Collection,
};
use serde::{Deserialize, Serialize};

pub struct Manager {
    client: mongodb::Client,
    pub database: mongodb::Database,
    users: mongodb::Collection<User>,
    messages: mongodb::Collection<Message>,
}

// mongodb collection struct here
#[derive(Serialize, Deserialize, Debug)]
struct User {
    #[serde(serialize_with = "serialize_hex_string_as_object_id")]
    id: String,
    name: String,
    email: String,
    password: String,
    avatar: String,
    friends: Vec<oid::ObjectId>,
    groups: Vec<oid::ObjectId>,
    chats: Vec<oid::ObjectId>,
    __v: u128,
}

#[derive(Serialize, Deserialize, Debug)]
struct Message {
    #[serde(serialize_with = "serialize_hex_string_as_object_id")]
    id: String,
    sender: oid::ObjectId,
    receiver: oid::ObjectId,
    #[serde(serialize_with = "serialize_bson_datetime_as_rfc3339_string")]
    time: DateTime,
    content: String,
    image: Vec<String>,
    __v: u128,
}

impl Manager {
    pub async fn new(mongodb_url: &str) -> Result<Manager, mongodb::error::Error> {
        println!("Start connecting the mongodb server");
        let mut client_options = ClientOptions::parse(mongodb_url).await?;
        client_options.app_name = Some("test".to_string());
        let client = Client::with_options(client_options)?;
        let database = client.database("test");
        let users: Collection<User> = database.collection("users");
        let messages: Collection<Message> = database.collection("messages");
        Ok(Manager {
            client,
            database,
            users,
            messages,
        })
    }
}

#[test]
fn test() {}
