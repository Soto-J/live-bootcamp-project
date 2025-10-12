use crate::domain::User;
use std::{
    collections::{hash_map, HashMap},
    default,
};

#[derive(Debug, PartialEq)]
pub enum UserStoreError {
    UserAlreadyExists,
    UserNotFound,
    InvalidCredentials,
    UnexpectedError,
}

#[derive(Default)]
pub struct HashmapUserStore {
    users: HashMap<String, User>,
}

impl HashmapUserStore {
    pub fn add_user(&mut self, user: User) -> Result<(), UserStoreError> {
        match self.users.entry(user.email()) {
            hash_map::Entry::Occupied(_) => Err(UserStoreError::UserAlreadyExists),
            hash_map::Entry::Vacant(entry) => {
                entry.insert(user);
                Ok(())
            }
        }

        // let email = user.email();
        // if self.users.contains_key(&email) {
        //     return Err(UserStoreError::UserAlreadyExists);
        // }

        // self.users.insert(email, user)

        // Ok(())
    }

    pub fn get_user(&self, email: &str) -> Result<User, UserStoreError> {
        if let Some(user) = self.users.get(email) {
            return Ok(user.clone());
        }

        Err(UserStoreError::UserNotFound)
    }

    pub fn validate_user(&self, email: &str, password: &str) -> Result<(), UserStoreError> {
        if let Some(user) = self.users.get(email) {
            if user.password() == password {
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

        db.add_user(user.clone()).unwrap();

        let after_insert = db.get_user(&email).unwrap();
        assert_eq!(&after_insert.email(), &email);
        assert_eq!(&after_insert.password(), &password);
    }

    // #[tokio::test]
    // async fn test_validate_user() {
    //     todo!()
    // }
}
