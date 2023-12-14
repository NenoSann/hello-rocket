use mongodb::{options::ClientOptions, Client};

pub struct Manager {
    client: mongodb::Client,
    pub database: mongodb::Database,
}

impl Manager {
    pub async fn new(mongodb_url: &str) -> Result<Manager, mongodb::error::Error> {
        println!("Start connecting the mongodb server");
        let mut client_options = ClientOptions::parse(mongodb_url).await?;
        client_options.app_name = Some("pinia-database".to_string());
        // how can i change the Result to the ClientOptions?
        let client = Client::with_options(client_options)?;
        let database = client.database("picnia-database");
        // List the names of the databases in that deployment.
        for db_name in client.list_database_names(None, None).await? {
            println!("{}", db_name);
        }
        Ok(Manager { client, database })
    }
    pub async fn test(&self) {
        println!("the test function!");
    }
}

#[test]
fn test() {}
