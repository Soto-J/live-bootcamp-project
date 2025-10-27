use std::collections::{hash_map::Entry, HashMap};

use crate::domain::{
    data_stores::{LoginAttemptId, TwoFACode, TwoFACodeStore, TwoFACodeStoreError},
    email::Email,
};

#[derive(Default, Clone, PartialEq)]
pub struct HashmapTwoFACodeStore {
    codes: HashMap<Email, (LoginAttemptId, TwoFACode)>,
}

#[async_trait::async_trait]
impl TwoFACodeStore for HashmapTwoFACodeStore {
    async fn add_code(
        &mut self,
        email: Email,
        login_attempt_id: LoginAttemptId,
        code: TwoFACode,
    ) -> Result<(), TwoFACodeStoreError> {
        match self.codes.entry(email) {
            Entry::Vacant(entry) => {
                entry.insert((login_attempt_id, code));
                Ok(())
            }
            Entry::Occupied(_) => Err(TwoFACodeStoreError::UnexpectedError),
        }
    }

    async fn remove_code(&mut self, email: &Email) -> Result<(), TwoFACodeStoreError> {
        match self.codes.remove(email) {
            Some(_) => Ok(()),
            None => Err(TwoFACodeStoreError::UnexpectedError),
        }
    }

    async fn get_code(
        &self,
        email: &Email,
    ) -> Result<(LoginAttemptId, TwoFACode), TwoFACodeStoreError> {
        match self.codes.get(email) {
            Some(data) => Ok((data.0.clone(), data.1.clone())),
            None => Err(TwoFACodeStoreError::LoginAttemptIdNotFound),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        api::helpers::get_random_email,
        domain::{
            data_stores::{LoginAttemptId, TwoFACode, TwoFACodeStore},
            email::Email,
        },
        services::HashmapTwoFACodeStore,
    };

    #[tokio::test]
    async fn test_add_code() {
        let mut two_fa_store = HashmapTwoFACodeStore::default();

        let email = Email::parse(get_random_email()).unwrap();
        let two_fa_code = TwoFACode::default();
        let login_attempt_id = LoginAttemptId::default();

        let response = two_fa_store
            .add_code(email, login_attempt_id, two_fa_code)
            .await;

        assert_eq!(response, Ok(()))
    }

    #[tokio::test]
    async fn test_remove_code() {}

    #[tokio::test]
    async fn test_get_code() {}
}
