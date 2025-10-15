use crate::domain::AuthAPIError;

use fake::{faker::internet::en, Fake};
use serde::Deserialize;
use std::ops::Range;

#[derive(Debug, Deserialize, Clone, PartialEq, Eq, Hash)]
pub struct Password(pub String);
impl Password {
    pub fn parse(password: String) -> Result<(), AuthAPIError> {
        if password.len() < 8 {
            return Err(AuthAPIError::InvalidCredentials);
        }

        Ok(())
    }
}

impl AsRef<str> for Password {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

#[tokio::test]
async fn should_return_valid_password() {
    let password: String = en::Password(Range { start: 8, end: 15 }).fake();
    let response = Password::parse(password);

    assert_eq!(response, Ok(()), "Expect return value to be Ok(())")
}

#[tokio::test]
async fn should_return_invalid_password() {
    let password: String = en::Password(Range { start: 0, end: 7 }).fake();

    print!("PASSWORD: {}", password);
    let response = Password::parse(password);
    assert_eq!(
        response,
        Err(AuthAPIError::InvalidCredentials),
        "Expect return value to be Ok(())"
    )
}
