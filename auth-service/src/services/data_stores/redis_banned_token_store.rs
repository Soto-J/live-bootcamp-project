use std::sync::Arc;

use redis::{Commands, Connection};
use tokio::sync::RwLock;

use crate::{
    domain::data_stores::{BannedTokenStore, BannedTokenStoreError},
    utils::constants::TOKEN_TTL_SECONDS,
};

const BANNED_TOKEN_KEY_PREFIX: &str = "banned_token:";

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
        let key = get_key(&token);

        let redis_connection = self.conn.clone();

        let redis_result = tokio::task::spawn_blocking(move || {
            redis_connection.blocking_write().set_ex(key, true, seconds)
        })
        .await
        .map_err(|_| BannedTokenStoreError::UnexpectedError)?;

        redis_result.map_err(|_| BannedTokenStoreError::UnexpectedError)
    }

    async fn contains_token(&self, token: &str) -> Result<bool, BannedTokenStoreError> {
        let key = get_key(token);
        let redis_connection = self.conn.clone();

        let redis_result =
            tokio::task::spawn_blocking(move || redis_connection.blocking_write().exists(key))
                .await
                .map_err(|_| BannedTokenStoreError::UnexpectedError)?;

        redis_result.map_err(|_| BannedTokenStoreError::UnexpectedError)
    }
}

// prefix to prevent collisions and organize data!
fn get_key(token: &str) -> String {
    format!("{}{}", BANNED_TOKEN_KEY_PREFIX, token)
}
