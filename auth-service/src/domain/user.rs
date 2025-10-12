#[derive()]
pub struct User {
    email: String,
    password: String,
    requires_2fa: bool,
}

impl User {
    pub fn new(email: String, password: String, requires_2fa: bool) -> Self {
        Self {
            email,
            password,
            requires_2fa,
        }
    }

    pub fn email(&self) -> &String {
        return &self.email;
    }
}
