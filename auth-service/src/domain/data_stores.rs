use crate::domain::{email::Email, password::Password, user::User};

use serde::{Deserialize, Serialize};
use uuid::Uuid;

// ~~~ User Store
#[derive(Debug, Clone, PartialEq)]
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

// ~~~ Banned token store
#[derive(Debug, Clone, PartialEq)]
pub enum BannedTokenStoreError {
    TokenNotFound,
    FailedToStoreToken,
    UnexpectedError,
}

#[async_trait::async_trait]
pub trait BannedTokenStore: Send + Sync {
    async fn add_token(&mut self, token: String) -> Result<(), BannedTokenStoreError>;
    async fn contains_token(&self, token: &str) -> Result<bool, BannedTokenStoreError>;
}

// ~~~ 2FA Store
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LoginAttemptId(String);

impl AsRef<str> for LoginAttemptId {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl Default for LoginAttemptId {
    fn default() -> Self {
        Self(Uuid::new_v4().to_string())
    }
}

impl PartialEq<LoginAttemptId> for String {
    fn eq(&self, other: &LoginAttemptId) -> bool {
        *self == other.0
    }
}

impl From<LoginAttemptId> for String {
    fn from(value: LoginAttemptId) -> Self {
        value.0
    }
}

impl LoginAttemptId {
    pub fn parse(id: String) -> Result<Self, String> {
        match Uuid::parse_str(&id) {
            Ok(id) => Ok(Self(id.to_string())),
            _ => Err(format!("Invalid UUID: {}", id)),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum TwoFACodeStoreError {
    LoginAttemptIdNotFound,
    UnexpectedError,
}

// This trait represents the interface all concrete 2FA code stores should implement
#[async_trait::async_trait]
pub trait TwoFACodeStore {
    async fn add_code(
        &mut self,
        email: Email,
        login_attempt_id: LoginAttemptId,
        code: TwoFACode,
    ) -> Result<(), TwoFACodeStoreError>;
    async fn remove_code(&mut self, email: &Email) -> Result<(), TwoFACodeStoreError>;
    async fn get_code(
        &self,
        email: &Email,
    ) -> Result<(LoginAttemptId, TwoFACode), TwoFACodeStoreError>;
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TwoFACode(String);

impl TwoFACode {
    pub fn parse(code: String) -> Result<Self, String> {
        if code.chars().all(|ch| !ch.is_ascii_digit()) {
            return Err(format!("Code must be numeric {}", code));
        }

        if code.len() < 6 {
            return Err(format!("Code must be numeric {}", code));
        }

        Ok(Self(code))
    }
}

impl AsRef<str> for TwoFACode {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl From<TwoFACode> for String {
    fn from(value: TwoFACode) -> Self {
        value.0
    }
}

impl Default for TwoFACode {
    fn default() -> Self {
        Self(Uuid::new_v4().to_string())
    }
}
