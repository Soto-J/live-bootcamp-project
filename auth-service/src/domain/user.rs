use serde::{Deserialize, Serialize};

use crate::domain::{email::Email, password::Password};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct User {
    email: Email,
    password: Password,
    #[serde(rename = "requires2FA")]
    requires_2fa: bool,
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

    pub fn password(&self) -> &Password {
        &self.password
    }

    pub fn has_2fa(&self) -> bool {
        self.requires_2fa
    }
}
