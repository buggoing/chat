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
        match bson::to_document(user) {
            Ok(doc) => match coll.insert_one(doc, None).await {
                Ok(_) => (),
                Err(e) => eprintln!("mongo failed to insert user err: {}", e),
            },
            Err(e) => return Err(Error::Convert(e.to_string())),
        }

        Ok(())
    }

    pub async fn get_user(&self, filter: bson::Document) -> Result<Vec<User>, Error> {
        let coll = self.db.collection(User::collection_name());
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

    pub async fn create_post(&self, post: &Post) -> Result<(), Error> {
        let coll = self.db.collection(Post::collection_name());
        match bson::to_document(post) {
            Ok(doc) => match coll.insert_one(doc, None).await {
                Ok(_) => (),
                Err(e) => {
                    eprintln!("mongo failed to create post err: {}", e);
                    return Err(Error::Operation);
                }
            },
            Err(e) => return Err(Error::Convert(e.to_string())),
        }
        Ok(())
    }

    pub async fn get_post(&self, filter: bson::Document) -> Result<Post, Error> {
        let coll = self.db.collection(Post::collection_name());
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

impl<'a> User {
    fn collection_name() -> &'a str {
        "user"
    }
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Post {
    id: String,
    title: String,
    content: String,
    tag: String,
}

impl<'a> Post {
    fn collection_name() -> &'a str {
        "post"
    }
}
