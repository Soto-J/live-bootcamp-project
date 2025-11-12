use color_eyre::eyre;
use secrecy::{ExposeSecret, Secret};
use serde::Deserialize;
use std::hash::Hash;

#[derive(Debug, Deserialize, Clone)]
pub struct Password(Secret<String>);
impl Password {
    pub fn parse(password: Secret<String>) -> eyre::Result<Password> {
        if Self::invalid_password(&password) {
            return Err(eyre::eyre!("Failed to parse string to a Password type"));
        }

        Ok(Self(password))
    }

    fn invalid_password(password: &Secret<String>) -> bool {
        if password.expose_secret().len() < 8 {
            return true;
        }

        false
    }
}

impl AsRef<Secret<String>> for Password {
    fn as_ref(&self) -> &Secret<String> {
        &self.0
    }
}

impl From<Secret<String>> for Password {
    fn from(value: Secret<String>) -> Self {
        Self(value)
    }
}

impl PartialEq for Password {
    fn eq(&self, other: &Self) -> bool {
        self.0.expose_secret() == other.0.expose_secret()
    }
}

impl Eq for Password {}

impl Hash for Password {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.0.expose_secret().hash(state);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::Email;

    use fake::faker::internet::en::SafeEmail;
    use fake::Fake;
    use secrecy::Secret;

    #[test]
    fn empty_string_is_rejected() {
        let password = Secret::new("".to_string());

        assert!(Password::parse(password).is_err());
    }

    #[test]
    fn string_less_than_8_characters_is_rejected() {
        let password = Secret::new("1234567".to_string());

        assert!(Password::parse(password).is_err());
    }

    #[derive(Debug, Clone)]
    struct ValidEmailFixture(pub String);

    impl quickcheck::Arbitrary for ValidEmailFixture {
        fn arbitrary<G: quickcheck::Gen>(g: &mut G) -> Self {
            let email = SafeEmail().fake_with_rng(g);
            Self(email)
        }
    }

    #[quickcheck_macros::quickcheck]
    fn valid_emails_are_parsed_successfully(valid_email: ValidEmailFixture) -> bool {
        Email::parse(Secret::new(valid_email.0)).is_ok()
    }
}
