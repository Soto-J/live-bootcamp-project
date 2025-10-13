#[derive(Clone, PartialEq, Debug)]
pub struct User {
    email: String,
    password: String,
    requires_2fa: bool,
}

impl User {
    pub fn new<T: AsRef<str>>(email: T, password: T, requires_2fa: bool) -> Self {
        Self {
            email: email.as_ref().to_owned(),
            password: password.as_ref().to_owned(),
            requires_2fa,
        }
    }

    pub fn email(&self) -> &str {
        &self.email
    }

    pub fn password(&self) -> &str {
        &self.password
    }
}
