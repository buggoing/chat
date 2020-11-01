use mongodb::{
    bson::{self, doc, Bson, Bson::Document},
    error::Error,
    options::ClientOptions,
    Client, Database,
};
use serde::{Deserialize, Serialize};
use std::vec::Vec;
use tokio::stream::StreamExt;

#[derive(Debug)]
pub struct DB {
    cli: Client,
    db: Database,
}

impl DB {
    pub async fn new(uri: &str, db_name: &str) -> Self {
        let mut client_options = match ClientOptions::parse(uri).await {
            Ok(options) => options,
            Err(e) => panic!("failed to parse mongo uri err: {}", e),
        };
        client_options.app_name = Some("chat".to_string());
        let client = match Client::with_options(client_options) {
            Ok(cli) => cli,
            Err(e) => panic!("failed to create mongo client err: {}", e),
        };
        let database = client.database(db_name);
        match client.list_database_names(None, None).await {
            Err(e) => panic!("failed to list mongo databases err: {}", e),
            Ok(_) => {}
        };
        Self {
            cli: client,
            db: database,
        }
    }

    pub async fn create_user(&self, user: &User) -> Result<(), Error> {
        let coll = self.db.collection(User::collection_name());
        let doc = bson::to_document(user)?;
        match coll.insert_one(doc, None).await {
            Ok(_) => (),
            Err(e) => eprintln!("mongo failed to insert user err: {}", e),
        }
        Ok(())
    }

    pub async fn get_user(&self, filter: bson::Document) -> Result<Vec<User>, Error> {
        let coll = self.db.collection(User::collection_name());
        let mut cursor = coll.find(filter, None).await?;
        let mut users = Vec::new();
        // Iterate over each document in the cursor, using serde to
        // deserialize them
        while let Some(res) = cursor.next().await {
            match res {
                Ok(doc) => {
                    let user: User = bson::from_bson(Bson::Document(doc))?;
                    users.push(user)
                }
                Err(e) => eprintln!("mongo cursor err: {}", e),
            }
        }
        Ok(users)
    }
}

#[derive(Deserialize, Serialize, Debug)]
pub struct User {
    pub name: String,
    pub create_time: i64,
}

impl<'a> User {
    fn collection_name() -> &'a str {
        "user"
    }
}
