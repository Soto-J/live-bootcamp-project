use color_eyre::eyre;
use secrecy::{ExposeSecret, Secret};
use serde::Deserialize;
use std::hash::Hash;
use validator::ValidateEmail;

#[derive(Debug, Clone, Deserialize)]
pub struct Email(Secret<String>);

impl Email {
    pub fn parse(email: Secret<String>) -> eyre::Result<Email> {
        if !ValidateEmail::validate_email(email.expose_secret()) {
            return Err(eyre::eyre!(
                "{} is not a valid email.",
                email.expose_secret()
            ));
        }
        Ok(Self(email))
    }
}

impl AsRef<Secret<String>> for Email {
    fn as_ref(&self) -> &Secret<String> {
        &self.0
    }
}

impl From<Secret<String>> for Email {
    fn from(value: Secret<String>) -> Self {
        Self(value)
    }
}

impl PartialEq for Email {
    fn eq(&self, other: &Self) -> bool {
        self.0.expose_secret() == other.0.expose_secret()
    }
}

impl Eq for Email {}

impl Hash for Email {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.0.expose_secret().hash(state);
    }
}

#[cfg(test)]
mod test {
    use fake::{faker::internet::en::SafeEmail, Fake};
    use secrecy::Secret;

    use super::Email;

    #[test]
    fn empty_string_is_rejected() {
        let email = "".to_string();

        assert!(Email::parse(email.into()).is_err());
    }

    #[test]
    fn email_missing_at_symbol_is_rejected() {
        let email = Secret::new("".to_string());

        assert!(Email::parse(email).is_err());
    }

    #[test]
    fn email_missing_subject_is_rejected() {
        let email = Secret::new("@domain.com".to_string());

        assert!(Email::parse(email).is_err());
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
