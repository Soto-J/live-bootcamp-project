use axum_extra::extract::cookie::Cookie;

use crate::domain::{Email, Password, User};

#[derive(Debug, PartialEq)]
pub enum UserStoreError {
    UserAlreadyExists,
    UserNotFound,
    InvalidCredentials,
    UnexpectedError,
    IncorrectCredentials,
}

#[async_trait::async_trait]
pub trait UserStore: Send + Sync {
    async fn add_user(&mut self, user: User) -> Result<(), UserStoreError>;
    async fn get_user(&self, email: &Email) -> Result<User, UserStoreError>;
    async fn validate_user(&self, email: &Email, password: &Password)
        -> Result<(), UserStoreError>;
}

#[derive(Debug, PartialEq)]
pub enum BannedTokenStoreError {
    TokenNotFound,
    FailedToStoreToken,
}

#[async_trait::async_trait]
pub trait BannedTokenStore: Send + Sync {
    fn store_token(&mut self, token: &str) -> Result<(), BannedTokenStoreError>;
    fn has_token(&self, token: &str) -> bool;
}
