use crate::domain::AuthAPIError;

use fake::{faker::internet::en::SafeEmail, Fake};
use validator::ValidateEmail;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Email(pub String);

impl Email {
    pub fn parse(email: String) -> Result<(), AuthAPIError> {
        let email = email.trim();
        let is_not_valid = email.is_empty()
            || !email.contains('.')
            || !email.contains('@')
            || !ValidateEmail::validate_email(&email);

        if is_not_valid {
            return Err(AuthAPIError::InvalidCredentials);
        }

        if let Some(at_index) = email.find('@') {
            let domain = &email[at_index + 1..];
            if !domain.contains('.') {
                return Err(AuthAPIError::InvalidCredentials);
            }
        }

        Ok(())
    }
}

impl AsRef<str> for Email {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

#[tokio::test]
async fn should_return_valid_email() {
    let test_cases: Vec<String> = vec![
        SafeEmail().fake(),
        SafeEmail().fake(),
        SafeEmail().fake(),
        SafeEmail().fake(),
        SafeEmail().fake(),
        SafeEmail().fake(),
        SafeEmail().fake(),
        SafeEmail().fake(),
        SafeEmail().fake(),
        SafeEmail().fake(),
    ];

    for test_case in test_cases {
        print!("testing - Email: {:?}", test_case);
        let response = Email::parse(test_case);
        assert_eq!(response, Ok(()), "Expect return value to be Ok(())")
    }
}

#[tokio::test]
async fn should_return_invalid_email() {
    let test_cases: Vec<String> = vec![
        String::from(""),
        String::from("test.com"),
        String::from("test@com."),
        String::from("te.st@com"),
    ];

    for test_case in test_cases {
        println!("testing - Email: {:?}", test_case);
        let response = Email::parse(test_case);
        assert_eq!(
            response,
            Err(AuthAPIError::InvalidCredentials),
            "Expect return value to be AuthAPIError InvalidCredentials"
        )
    }
}
