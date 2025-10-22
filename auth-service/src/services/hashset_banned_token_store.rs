use std::collections::HashSet;

use crate::domain::{BannedTokenStore, BannedTokenStoreError};

#[derive(Default, PartialEq, Clone)]
pub struct HashsetBannedTokenStore {
    tokens: HashSet<String>,
}

impl BannedTokenStore for HashsetBannedTokenStore {
    fn store_token(&mut self, token: &str) -> Result<(), BannedTokenStoreError> {
        match self.tokens.insert(token.into()) {
            true => Ok(()),
            false => Err(BannedTokenStoreError::FailedToStoreToken),
        }
    }

    fn has_token(&self, token: &str) -> bool {
        self.tokens.contains(token)
    }
}

#[cfg(test)]
mod test {
    use crate::{domain::BannedTokenStore, services::HashsetBannedTokenStore};

    #[test]
    fn test_store_token() {
        let mut banned_token_store = HashsetBannedTokenStore::default();
        let token = "token";

        let response = banned_token_store.store_token(token);

        assert_eq!(response, Ok(()))
    }

    #[test]
    fn test_has_toke() {
        let mut banned_token_store = HashsetBannedTokenStore::default();
        let token = "token";

        banned_token_store.store_token(token).unwrap();

        assert!(banned_token_store.has_token(token))
    }
}
