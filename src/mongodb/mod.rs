use mongodb::{options::ClientOptions, Client};

pub struct Manager {
    client: mongodb::Client,
}

impl Manager {
    pub async fn new(mongodb_url: &str) -> Result<Manager, mongodb::error::Error> {
        let client_options = ClientOptions::parse(mongodb_url)
            .await
            .unwrap_or_else(|error| panic!("Error connect to mongodb!\n {}", error));
        // how can i change the Result to the ClientOptions?
        let client = Client::with_options(client_options)?;
        for db_name in client.list_database_names(None, None).await? {
            println!("{}", db_name);
        }
        Ok(Manager { client })
    }
}

#[test]
fn test() {}
