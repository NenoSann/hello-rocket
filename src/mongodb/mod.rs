use std::thread::current;

use mongodb::{
    bson::{doc, Document},
    error::Error,
    options::ClientOptions,
    Client, Collection,
};
use rocket::futures::StreamExt;

pub struct Manager {
    client: mongodb::Client,
    pub database: mongodb::Database,
}

impl Manager {
    pub async fn new(mongodb_url: &str) -> Result<Manager, mongodb::error::Error> {
        println!("Start connecting the mongodb server");
        let mut client_options = ClientOptions::parse(mongodb_url).await?;
        client_options.app_name = Some("test".to_string());
        let client = Client::with_options(client_options)?;
        let database = client.database("test");
        Ok(Manager { client, database })
    }

    pub async fn test(&self) {
        let collection: Collection<Document> = self.database.collection("users");
        let user = collection
            .find_one(doc! {"name":"NenoSan"}, None)
            .await
            .unwrap();
        match user {
            Some(user) => {
                println!("{:?}", user);
            }
            None => println!("User not found"),
        }
    }
}

#[test]
fn test() {}
