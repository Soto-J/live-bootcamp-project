use crate::domain::{email::Email, password::Password};

use serde::Deserialize;

#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct User {
    pub email: Email,
    pub password: Password,
    #[serde(rename = "requires2FA")]
    pub requires_2fa: bool,
}

impl User {
    pub fn new(email: Email, password: Password, requires_2fa: bool) -> Self {
        Self {
            email,
            password,
            requires_2fa,
        }
    }

    pub fn email(&self) -> &Email {
        &self.email
    }

    pub fn has_2fa(&self) -> bool {
        self.requires_2fa
    }
}
