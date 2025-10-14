use crate::{api::get_random_email, domain::AuthAPIError};

pub struct Email(String);

impl Email {
    pub fn parse(email: String) -> Result<(), AuthAPIError> {
        if !email.contains('@') {
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
    let response = Email::parse(get_random_email());

    assert_eq!(response, Ok(()))
}
