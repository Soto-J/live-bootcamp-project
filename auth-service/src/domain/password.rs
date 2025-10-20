use serde::Deserialize;

#[derive(Debug, Deserialize, Clone, PartialEq, Eq, Hash)]
pub struct Password(pub String);
impl Password {
    pub fn parse(password: String) -> Result<Password, String> {
        if Self::invalid_password(&password) {
            return Err("Failed to parse string to a Password type".into());
        }

        Ok(Self(password))
    }

    fn invalid_password(password: &str) -> bool {
        if password.len() < 8 {
            return true;
        }

        false
    }
}

impl AsRef<str> for Password {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

#[cfg(test)]
mod tests {
    use crate::api::{get_invalid_password, get_random_password};

    use super::Password;
    use fake::faker::internet::en::Password as FakePassword;
    use fake::Fake;

    #[test]
    fn empty_string_is_rejected() {
        let password = "".to_owned();
        assert!(Password::parse(password).is_err());
    }

    #[test]
    fn string_less_than_8_characters_is_rejected() {
        let password = "1234567".to_owned();
        assert!(Password::parse(password).is_err());
    }

    // #[derive(Debug, Clone)]
    // struct ValidPasswordFixture(pub String);

    // impl quickcheck::Arbitrary for ValidPasswordFixture {
    //     fn arbitrary<G: quickcheck::Gen>(g: &mut G) -> Self {
    //         let password = FakePassword(8..30).fake_with_rng(g);
    //         Self(password)
    //     }
    // }

    // #[quickcheck_macros::quickcheck]
    // fn valid_passwords_are_parsed_successfully(valid_password: ValidPasswordFixture) -> bool {
    //     Password::parse(valid_password.0).is_ok()
    // }
}
