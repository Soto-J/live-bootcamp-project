use std::sync::Arc;

use color_eyre::eyre::Context;
use redis::{Commands, Connection};
use secrecy::{ExposeSecret, Secret};
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
    #[tracing::instrument(name = "Add_Token", skip_all)]
    async fn add_token(&mut self, token: Secret<String>) -> Result<(), BannedTokenStoreError> {
        let expired_in: u64 = TOKEN_TTL_SECONDS
            .try_into()
            .wrap_err("Failed to create expiration token.")
            .map_err(BannedTokenStoreError::UnexpectedError)?;

        let token_key = get_key(token.expose_secret());

        let _: () = self
            .conn
            .write()
            .await
            .set_ex(token_key, true, expired_in)
            .wrap_err("Failed to set expiration token.")
            .map_err(BannedTokenStoreError::UnexpectedError)?;

        Ok(())
    }
    #[tracing::instrument(name = "Contains_Token", skip_all)]
    async fn contains_token(&self, token: &Secret<String>) -> Result<bool, BannedTokenStoreError> {
        let token_key = get_key(token.expose_secret());

        let is_banned = self
            .conn
            .write()
            .await
            .exists(&token_key)
            .wrap_err("Failed to check if token is banned.")
            .map_err(BannedTokenStoreError::UnexpectedError)?;

        Ok(is_banned)
    }
}

// prefix to prevent collisions and organize data!
fn get_key(token: &str) -> String {
    format!("{}{}", BANNED_TOKEN_KEY_PREFIX, token)
}
