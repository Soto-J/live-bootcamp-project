use crate::domain::{User, UserStore, UserStoreError};

use std::collections::{hash_map::Entry, HashMap};

#[derive(Default, Clone)]
pub struct HashmapUserStore {
    users: HashMap<String, User>,
}

#[async_trait::async_trait]
impl UserStore for HashmapUserStore {
    async fn add_user(&mut self, user: User) -> Result<(), UserStoreError> {
        match self.users.entry(user.email().to_string()) {
            Entry::Occupied(_) => Err(UserStoreError::UserAlreadyExists),
            Entry::Vacant(entry) => {
                entry.insert(user);
                Ok(())
            }
        }
    }

    async fn get_user(&self, email: &str) -> Result<User, UserStoreError> {
        self.users
            .get(email)
            .cloned()
            .ok_or(UserStoreError::UserNotFound)
    }

    async fn validate_user(&self, email: &str, password: &str) -> Result<(), UserStoreError> {
        let user = self.get_user(&email).await?;

        if user.password() != password {
            return Err(UserStoreError::InvalidCredentials);
        }

        Ok(())
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

        let result = db.add_user(user.clone()).await;

        assert_eq!(
            result,
            Ok(()),
            "Expected adding a new user to succeed, but it failed."
        );

        let result = db.add_user(user).await;

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

        let before_insert = db.get_user(&email).await;

        assert_eq!(
            before_insert,
            Err(UserStoreError::UserNotFound),
            "Expected get user to return UserNotFound."
        );

        let password = get_random_password();
        let user = User::new(email.clone(), password.clone(), false);

        db.add_user(user.clone())
            .await
            .expect("Failed to insert test user.");

        let after_insert = db
            .get_user(&email)
            .await
            .expect("Failed to insert test user 2.");

        assert_eq!(&after_insert.email(), &email);
        assert_eq!(&after_insert.password(), &password);
    }

    #[tokio::test]
    async fn test_validate_user() {
        let mut db = HashmapUserStore::default();

        let email = get_random_email();
        let password = get_random_password();

        let user = User::new(&email, &password, false);

        db.add_user(user).await.expect("Failed to add test user.");

        let correct_credentials = db.validate_user(&email, &password).await;
        assert_eq!(correct_credentials, Ok(()), "Expect to return Ok(()).");

        let incorrect_email = db.validate_user(&get_random_email(), &password).await;
        assert_eq!(
            incorrect_email,
            Err(UserStoreError::UserNotFound),
            "Expected to return UserNotFound with incorrect email."
        );

        let incorrect_credentials = db
            .validate_user(&get_random_email(), &get_random_password())
            .await;
        assert_eq!(
            incorrect_credentials,
            Err(UserStoreError::UserNotFound),
            "Expected to return UserNotFound with incorrect email and password."
        );

        let incorrect_password = db.validate_user(&email, &get_random_password()).await;
        assert_eq!(
            incorrect_password,
            Err(UserStoreError::InvalidCredentials),
            "Expected to return InvalidCredentials with incorrect password."
        )
    }
}
