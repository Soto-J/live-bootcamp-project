use std::collections::HashMap;

use crate::domain::{
    data_stores::{LoginAttemptId, TwoFACode, TwoFACodeStore, TwoFACodeStoreError},
    email::Email,
};

#[derive(Default, Debug, Clone, PartialEq)]
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
        self.codes.insert(email, (login_attempt_id, code));

        Ok(())
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
    use super::*;
    
    use crate::api::helpers::get_random_email;

    #[tokio::test]
    async fn test_add_code() {
        let mut two_fa_store = HashmapTwoFACodeStore::default();

        let email = Email::parse(get_random_email()).unwrap();
        let two_fa_code = TwoFACode::default();
        let login_attempt_id = LoginAttemptId::default();

        let response = two_fa_store
            .add_code(email, login_attempt_id, two_fa_code)
            .await;

        assert!(response.is_ok())
    }

    #[tokio::test]
    async fn test_remove_code() {
        let mut two_fa_store = HashmapTwoFACodeStore::default();

        let email = Email::parse(get_random_email()).unwrap();
        let two_fa_code = TwoFACode::default();
        let login_attempt_id = LoginAttemptId::default();

        let response = two_fa_store
            .add_code(email.clone(), login_attempt_id, two_fa_code)
            .await;

        assert!(response.is_ok());

        let response = two_fa_store.remove_code(&email).await;

        assert!(response.is_ok())
    }

    #[tokio::test]
    async fn test_get_code() {
        let mut two_fa_store = HashmapTwoFACodeStore::default();

        let email = Email::parse(get_random_email()).unwrap();
        let two_fa_code = TwoFACode::default();
        let login_attempt_id = LoginAttemptId::default();

        let add_code_response = two_fa_store
            .add_code(email.clone(), login_attempt_id, two_fa_code)
            .await;

        assert!(add_code_response.is_ok());

        let get_code_response = two_fa_store.get_code(&email).await;

        assert!(get_code_response.is_ok());

        let email = Email::parse(get_random_email()).unwrap();
        // in the case code doesnt exist
        let empty_code_response = two_fa_store.get_code(&email).await;
        assert_eq!(
            empty_code_response,
            Err(TwoFACodeStoreError::LoginAttemptIdNotFound)
        )
    }
}
