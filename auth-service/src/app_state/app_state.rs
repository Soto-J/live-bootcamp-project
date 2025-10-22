use crate::domain::{BannedTokenStore, UserStore};

use std::sync::Arc;
use tokio::sync::RwLock;

pub type UserStoreType = Arc<RwLock<dyn UserStore + Send + Sync>>;
pub type BannedTokenStoreType = Arc<RwLock<dyn BannedTokenStore + Send + Sync>>;

#[derive(Clone)]
pub struct AppState {
    pub user_store: UserStoreType,
    pub banned_store: BannedTokenStoreType,
}

impl AppState {
    pub fn new(user_store: UserStoreType, banned_store: BannedTokenStoreType) -> Self {
        Self {
            user_store,
            banned_store,
        }
    }
}
