use crate::{
    api::{get_random_email, get_random_password},
    domain::{validate_email, AuthAPIError},
};
use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Email(pub String);

#[derive(Debug, Validate)]
struct WrapperEmail {
    #[validate(custom(function = "validate_email"))]
    email: String,
}

impl Email {
    pub fn parse(email: String) -> Result<(), AuthAPIError> {
        if let Err(_) = validate_email(&email) {
            return Err(AuthAPIError::InvalidCredentials);
        }

        Ok(())
    }
}

impl AsRef<str> for Email {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

pub struct Password(String);

impl Password {
    pub fn parse(password: String) -> Result<(), AuthAPIError> {
        if password.len() < 8 {
            return Err(AuthAPIError::InvalidCredentials);
        }

        Ok(())
    }
}

#[tokio::test]
async fn should_return_valid_email() {
    // let response = Email::parse(get_random_email());
    let response = Email::parse("get_random_email()".to_string());

    assert_eq!(response, Ok(()), "Expect return value to be Ok(())")
}

#[tokio::test]
async fn should_return_valid_password() {
    let response = Password::parse(get_random_password());

    assert_eq!(response, Ok(()), "Expect return value to be Ok(())")
}
