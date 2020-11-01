mod db;

use db::mongo;
use mongodb::{bson::doc, error::Result, options::ClientOptions, Client};
use std::time::{SystemTime, UNIX_EPOCH};

#[tokio::main]
async fn main() -> Result<()> {
    let mongo_uri = "mongodb://chat:chat@localhost:27017/?directconnection=true";
    let mongo_db_name = "chat";
    let mongo_client = mongo::DB::new(mongo_uri, mongo_db_name).await;
    println!("Hello, world!");

    // let user = mongo::User {
    //     name: "name".to_string(),
    //     create_time: SystemTime::now()
    //         .duration_since(UNIX_EPOCH)
    //         .unwrap()
    //         .as_secs() as i64,
    // };

    // match mongo_client.create_user(&user).await {
    //     Err(e) => eprintln!("failed to create user err: {}", e),
    //     Ok(()) => (),
    // }
    let filter = doc! {"name": "name"};
    match mongo_client.get_user(filter).await {
        Ok(users) => println!("users: {:?}", users),
        Err(e) => eprintln!("failed to get users err: {}", e),
    }

    let redis_uri = "redis://127.0.0.1";
    let redis_client = db::redis::mredis::new(redis_uri);
    match redis_client.set("hello".to_string(), "world".to_string(), 0) {
        Ok(_) => {}
        Err(e) => eprintln!("failed to set err: {}", e),
    }

    match redis_client.get("hello") {
        Ok(val) => println!("get redis value {}", val),
        Err(e) => eprintln!("failed to get by key err: {}", e),
    }
    Ok(())
}
