use redis::{Client, Commands, RedisResult};
use std::time::Duration;

#[derive(Debug)]
pub struct mredis {
    cli: Client,
}

impl mredis {
    pub fn new(uri: &str) -> Self {
        let cli = match redis::Client::open("redis://127.0.0.1/") {
            Ok(cli) => cli,
            Err(e) => panic!("failed to open redis err: {}", e),
        };
        Self { cli: cli }
    }

    pub fn set(&self, key: String, value: String, seconds: usize) -> RedisResult<()> {
        let mut conn = self.cli.get_connection()?;
        if seconds > 0 {
            conn.set_ex(key, value, seconds)?;
        } else {
            conn.set(key, value)?;
        }
        Ok(())
    }

    pub fn get(&self, key: &str) -> RedisResult<String> {
        let mut conn = self.cli.get_connection()?;
        let res: String = conn.get(key)?;
        Ok(res)
    }
}
