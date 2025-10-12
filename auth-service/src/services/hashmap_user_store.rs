use std::{
    collections::{hash_map, HashMap},
    default,
};

use axum::http::header::Entry;

use crate::domain::User;

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
        let email = user.email().clone();

        match self.users.entry(email) {
            hash_map::Entry::Occupied(_) => Err(UserStoreError::UserAlreadyExists),
            hash_map::Entry::Vacant(entry) => {
                entry.insert(user);
                Ok(())
            }
        }

        // if self.users.contains_key(&email) {
        //     return Err(UserStoreError::UserAlreadyExists);
        // }

        // self.users
        //     .insert(email, user)
        //     .expect("Failed to insert user.");

        // Ok(())
    }

    pub fn get_user(&self, email: &str) -> Result<(), User> {
        
    }

    // TODO: Implement a public method called `get_user`, which takes an
    // immutable reference to self and an email string slice as arguments.
    // This function should return a `Result` type containing either a
    // `User` object or a `UserStoreError`.
    // Return `UserStoreError::UserNotFound` if the user can not be found.

    // TODO: Implement a public method called `validate_user`, which takes an
    // immutable reference to self, an email string slice, and a password string slice
    // as arguments. `validate_user` should return a `Result` type containing either a
    // unit type `()` if the email/password passed in match an existing user, or a `UserStoreError`.
    // Return `UserStoreError::UserNotFound` if the user can not be found.
    // Return `UserStoreError::InvalidCredentials` if the password is incorrect.
}

// TODO: Add unit tests for your `HashmapUserStore` implementation
#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_add_user() {
        todo!()
    }

    #[tokio::test]
    async fn test_get_user() {
        todo!()
    }

    #[tokio::test]
    async fn test_validate_user() {
        todo!()
    }
}
