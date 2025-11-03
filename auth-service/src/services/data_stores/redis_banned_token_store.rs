use std::sync::Arc;

use redis::{Commands, Connection, RedisError};
use tokio::sync::RwLock;

use crate::{
    domain::data_stores::{BannedTokenStore, BannedTokenStoreError},
    utils::constants::TOKEN_TTL_SECONDS,
};

pub struct RedisBannedTokenStore {
    conn: Arc<RwLock<Connection>>,
}

impl RedisBannedTokenStore {
    pub fn new(conn: Arc<RwLock<Connection>>) -> Self {
        Self { conn }
    }
}

#[async_trait::async_trait]
impl BannedTokenStore for RedisBannedTokenStore {
    async fn add_token(&mut self, token: String) -> Result<(), BannedTokenStoreError> {
        let seconds: u64 = TOKEN_TTL_SECONDS
            .try_into()
            .map_err(|_| BannedTokenStoreError::UnexpectedError)?;
        let conn = self.conn.clone();
        let key = get_key(&token);

        tokio::task::spawn_blocking(move || {
            let mut connection = conn.blocking_write();
            let _: () = connection.set_ex(key, true, seconds)?;

            Ok::<(), RedisError>(())
        })
        .await
        .map_err(|_| BannedTokenStoreError::UnexpectedError)?
        .map_err(|_| BannedTokenStoreError::UnexpectedError)?;

        Ok(())
    }

    async fn contains_token(&self, token: &str) -> Result<bool, BannedTokenStoreError> {
        let key = get_key(token);
        let conn = self.conn.clone();

        let exists = tokio::task::spawn_blocking(move || {
            let mut connection = conn.blocking_write();

            let token_exist = connection.exists(&key)?;

            Ok::<bool, RedisError>(token_exist)
        })
        .await
        .map_err(|_| BannedTokenStoreError::UnexpectedError)?
        .map_err(|_| BannedTokenStoreError::UnexpectedError)?;

        Ok(exists)
    }
}

// We are using a key prefix to prevent collisions and organize data!
const BANNED_TOKEN_KEY_PREFIX: &str = "banned_token:";

fn get_key(token: &str) -> String {
    format!("{}{}", BANNED_TOKEN_KEY_PREFIX, token)
}
