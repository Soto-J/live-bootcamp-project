use crate::domain::{email::Email, password::Password, user::User};

use color_eyre::eyre::{self, eyre, Context, Ok};
use secrecy::{ExposeSecret, Secret};
use serde::Deserialize;
use uuid::Uuid;

// ~~~ User Store
#[derive(Debug, thiserror::Error)]
pub enum UserStoreError {
    #[error("User already exists")]
    UserAlreadyExists,
    #[error("User not found")]
    UserNotFound,
    #[error("Invalid credentials")]
    InvalidCredentials,
    #[error("Incorrect credientials")]
    IncorrectCredentials,
    #[error("Unexpected error")]
    UnexpectedError(#[source] eyre::Report),
}

impl PartialEq for UserStoreError {
    fn eq(&self, other: &Self) -> bool {
        core::mem::discriminant(self) == core::mem::discriminant(other)

        // matches!(
        //     (self, other),
        //     (Self::UserAlreadyExists, Self::UserAlreadyExists)
        //         | (Self::UserNotFound, Self::UserNotFound)
        //         | (Self::InvalidCredentials, Self::InvalidCredentials)
        //         | (Self::UnexpectedError(_), Self::UnexpectedError(_))
        // )
    }
}

#[async_trait::async_trait]
pub trait UserStore: Send + Sync {
    async fn add_user(&mut self, user: User) -> Result<(), UserStoreError>;
    async fn get_user(&self, email: &Email) -> Result<User, UserStoreError>;
    async fn validate_user(&self, email: &Email, password: &Password)
        -> Result<(), UserStoreError>;
}

// ~~~ Banned token store
#[derive(Debug, thiserror::Error)]
pub enum BannedTokenStoreError {
    #[error("Unexpected error")]
    UnexpectedError(#[source] eyre::Report),
}

#[async_trait::async_trait]
pub trait BannedTokenStore: Send + Sync {
    async fn add_token(&mut self, token: Secret<String>) -> Result<(), BannedTokenStoreError>;
    async fn contains_token(&self, token: &Secret<String>) -> Result<bool, BannedTokenStoreError>;
}

// ~~~ 2FA Store
#[derive(Debug, Clone, Deserialize)]
pub struct LoginAttemptId(Secret<String>);

impl LoginAttemptId {
    pub fn parse(id: Secret<String>) -> eyre::Result<Self> {
        let id = id.expose_secret();
        let parse_id = uuid::Uuid::parse_str(&id).wrap_err("Invalid login attempt id")?;

        Ok(Self(Secret::new(parse_id.to_string())))
    }
}

impl AsRef<Secret<String>> for LoginAttemptId {
    fn as_ref(&self) -> &Secret<String> {
        &self.0
    }
}

impl From<Secret<String>> for LoginAttemptId {
    fn from(value: Secret<String>) -> Self {
        Self(value)
    }
}

impl PartialEq for LoginAttemptId {
    fn eq(&self, other: &Self) -> bool {
        self.0.expose_secret() == other.0.expose_secret()
    }
}

impl Eq for LoginAttemptId {}

impl Default for LoginAttemptId {
    fn default() -> Self {
        Self(Secret::new(Uuid::new_v4().to_string()))
    }
}

#[derive(Debug, thiserror::Error)]
pub enum TwoFACodeStoreError {
    #[error("Login Attempt ID not found")]
    LoginAttemptIdNotFound,
    #[error("Unexpected error")]
    UnexpectedError(#[source] eyre::Report),
}
impl PartialEq for TwoFACodeStoreError {
    fn eq(&self, other: &Self) -> bool {
        core::mem::discriminant(self) == core::mem::discriminant(other)
    }
}

#[async_trait::async_trait]
pub trait TwoFACodeStore {
    async fn add_code(
        &mut self,
        email: Email,
        login_attempt_id: LoginAttemptId,
        code: TwoFACode,
    ) -> Result<(), TwoFACodeStoreError>;
    async fn remove_code(&mut self, email: &Email) -> std::result::Result<(), TwoFACodeStoreError>;
    async fn get_code(
        &self,
        email: &Email,
    ) -> Result<(LoginAttemptId, TwoFACode), TwoFACodeStoreError>;
}

#[derive(Debug, Clone, Deserialize)]
pub struct TwoFACode(Secret<String>);

impl TwoFACode {
    pub fn parse(code: String) -> eyre::Result<Self> {
        let code_as_u32 = code.parse::<u32>().wrap_err("Invalid 2FA code")?;

        if (100_000..=999_999).contains(&code_as_u32) {
            Ok(Self(Secret::new(code)))
        } else {
            Err(eyre!("Invalid 2Fa code"))
        }
    }
}

impl AsRef<Secret<String>> for TwoFACode {
    fn as_ref(&self) -> &Secret<String> {
        &self.0
    }
}

impl From<Secret<String>> for TwoFACode {
    fn from(value: Secret<String>) -> Self {
        Self(value)
    }
}

impl PartialEq for TwoFACode {
    fn eq(&self, other: &Self) -> bool {
        self.0.expose_secret() == other.0.expose_secret()
    }
}

impl Eq for TwoFACode {}

impl Default for TwoFACode {
    fn default() -> Self {
        Self(Secret::new(Uuid::new_v4().to_string()))
    }
}
