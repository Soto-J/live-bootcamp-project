use crate::domain::{Email, Password};

#[derive(Clone, PartialEq, Debug)]
pub struct User {
    email: Email,
    password: Password,
    requires_2fa: bool,
}

impl User {
    pub fn new(email: String, password: String, requires_2fa: bool) -> Self {
        Self {
            email: Email(email),
            password: Password(password),
            requires_2fa,
        }
    }

    pub fn email(&self) -> &Email {
        &self.email
    }

    pub fn password(&self) -> &Password {
        &self.password
    }
}
