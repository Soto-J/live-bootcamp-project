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
        let expired_in: u64 = TOKEN_TTL_SECONDS
            .try_into()
            .map_err(|_| BannedTokenStoreError::UnexpectedError)?;

        let token_key = get_key(&token);

        let _: () = self
            .conn
            .write()
            .await
            .set_ex(token_key, true, expired_in)
            .map_err(|_| BannedTokenStoreError::UnexpectedError)?;

        Ok(())
    }

    async fn contains_token(&self, token: &str) -> Result<bool, BannedTokenStoreError> {
        let token_key = get_key(token);

        let is_banned = self
            .conn
            .write()
            .await
            .exists(&token_key)
            .map_err(|_| BannedTokenStoreError::UnexpectedError)?;

        Ok(is_banned)
    }
}

// prefix to prevent collisions and organize data!
fn get_key(token: &str) -> String {
    format!("{}{}", BANNED_TOKEN_KEY_PREFIX, token)
}
