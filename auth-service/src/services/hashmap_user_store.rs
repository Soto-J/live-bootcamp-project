use crate::domain::User;

use std::collections::{hash_map::Entry, HashMap};

#[derive(Debug, PartialEq)]
pub enum UserStoreError {
    UserAlreadyExists,
    UserNotFound,
    InvalidCredentials,
    UnexpectedError,
}

#[allow(dead_code)]
#[derive(Default)]
pub struct HashmapUserStore {
    users: HashMap<String, User>,
}

impl HashmapUserStore {
    pub fn add_user(&mut self, user: User) -> Result<(), UserStoreError> {
        match self.users.entry(user.email().to_owned()) {
            Entry::Occupied(_) => Err(UserStoreError::UserAlreadyExists),
            Entry::Vacant(entry) => {
                entry.insert(user);
                Ok(())
            }
        }
    }

    pub fn get_user(&self, email: &str) -> Result<User, UserStoreError> {
        if let Some(user) = self.users.get(email) {
            return Ok(user.clone());
        }

        Err(UserStoreError::UserNotFound)
    }

    pub fn validate_user<T: AsRef<str>>(
        &self,
        email: T,
        password: T,
    ) -> Result<(), UserStoreError> {
        if let Some(user) = self.users.get(email.as_ref()) {
            if user.password() == password.as_ref() {
                return Ok(());
            } else {
                return Err(UserStoreError::InvalidCredentials);
            }
        }

        Err(UserStoreError::UserNotFound)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::api::{get_random_email, get_random_password};

    #[tokio::test]
    async fn test_add_user() {
        let mut db = HashmapUserStore::default();

        let email = get_random_email();
        let password = get_random_password();
        let user = User::new(email, password, false);

        let result = db.add_user(user.clone());
        assert_eq!(
            result,
            Ok(()),
            "Expected adding a new user to succeed, but it failed."
        );

        let result = db.add_user(user);
        assert_eq!(
            result,
            Err(UserStoreError::UserAlreadyExists),
            "Expected adding the same user again to return UserAlreadyExists error."
        )
    }

    #[tokio::test]
    async fn test_get_user() {
        let mut db = HashmapUserStore::default();
        let email = get_random_email();

        let before_insert = db.get_user(&email);
        assert_eq!(
            before_insert,
            Err(UserStoreError::UserNotFound),
            "Expected get user to return UserNotFound."
        );

        let password = get_random_password();
        let user = User::new(email.clone(), password.clone(), false);

        db.add_user(user.clone())
            .expect("Failed to insert test user.");

        let after_insert = db.get_user(&email).unwrap();
        assert_eq!(&after_insert.email(), &email);
        assert_eq!(&after_insert.password(), &password);
    }

    #[tokio::test]
    async fn test_validate_user() {
        let mut db = HashmapUserStore::default();

        let email = get_random_email();
        let password = get_random_password();

        let user = User::new(&email, &password, false);

        db.add_user(user).expect("Failed to add test user.");

        let correct_credentials = db.validate_user(&email, &password);
        assert_eq!(correct_credentials, Ok(()), "Expect to return Ok(()).");

        let incorrect_email = db.validate_user(get_random_email(), password);
        assert_eq!(
            incorrect_email,
            Err(UserStoreError::UserNotFound),
            "Expected to return UserNotFound with incorrect email."
        );

        let incorrect_credentials = db.validate_user(get_random_email(), get_random_password());
        assert_eq!(
            incorrect_credentials,
            Err(UserStoreError::UserNotFound),
            "Expected to return UserNotFound with incorrect email and password."
        );

        let incorrect_password = db.validate_user(&email, &get_random_password());
        assert_eq!(
            incorrect_password,
            Err(UserStoreError::InvalidCredentials),
            "Expected to return InvalidCredentials with incorrect password."
        )
    }
}
