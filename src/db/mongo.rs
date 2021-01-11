use super::error::Error;
use mongodb::{
    bson::{self, doc, Bson, Bson::Document},
    // error::Error,
    options::ClientOptions,
    Client,
    Database,
};
use serde::{Deserialize, Serialize};
use std::vec::Vec;
use tokio::stream::StreamExt;

#[derive(Debug)]
pub struct DB {
    cli: Client,
    db: Database,
}

pub trait Collection {
    fn collection_name() -> &'static str;
}

impl<'a> DB {
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

    pub async fn create_one<T>(&self, data: &T) -> Result<(), Error>
    where
        T: Serialize + Collection,
    {
        let coll = self.db.collection(T::collection_name());
        match bson::to_document(data) {
            Ok(doc) => match coll.insert_one(doc, None).await {
                Ok(_) => (),
                Err(e) => {
                    eprintln!("mongo failed to insert collection err: {}", e);
                    return Err(Error::Operation);
                }
            },
            Err(e) => return Err(Error::Convert(e.to_string())),
        }

        Ok(())
    }

    pub async fn get_some<T>(&self, filter: bson::Document) -> Result<Vec<T>, Error>
    where
        T: Serialize + for<'de> Deserialize<'de> + Collection,
    {
        let coll = self.db.collection(T::collection_name());
        match coll.find(filter, None).await {
            Ok(mut cursor) => {
                let mut users = Vec::new();
                // Iterate over each document in the cursor, using serde to
                // deserialize them
                while let Some(res) = cursor.next().await {
                    match res {
                        Ok(doc) => match bson::from_bson(Bson::Document(doc)) {
                            Ok(user) => users.push(user),
                            Err(e) => return Err(Error::Convert(e.to_string())),
                        },
                        Err(e) => eprintln!("mongo cursor err: {}", e),
                    }
                }
                Ok(users)
            }
            Err(e) => {
                eprintln!("mongo cursor err: {}", e);
                return Err(Error::Operation);
            }
        }
    }

    pub async fn find_one<T>(&self, filter: bson::Document) -> Result<T, Error>
    where
        T: Serialize + Collection + for<'de> Deserialize<'de>,
    {
        let coll = self.db.collection(T::collection_name());
        match coll.find_one(filter, None).await {
            Ok(doc) => {
                if let Some(doc) = doc {
                    match bson::from_bson(Bson::Document(doc)) {
                        Ok(post) => return Ok(post),
                        Err(e) => return Err(Error::Convert(e.to_string())),
                    }
                } else {
                    return Err(Error::NoRecord);
                }
            }
            Err(e) => {
                eprintln!("mongo failed to get post err: {}", e);
                return Err(Error::Operation);
            }
        }
    }
}

#[derive(Deserialize, Serialize, Debug)]
pub struct User {
    pub name: String,
    pub create_time: i64,
}

impl Collection for User {
    fn collection_name() -> &'static str {
        "user"
    }
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Post {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<bson::oid::ObjectId>,
    pub uid: i64,
    pub title: String,
    pub content: String,
    pub tag: String,
}

impl Collection for Post {
    fn collection_name() -> &'static str {
        "post"
    }
}
