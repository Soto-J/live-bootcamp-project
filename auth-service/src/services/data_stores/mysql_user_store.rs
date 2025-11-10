use crate::domain::{
    data_stores::{UserStore, UserStoreError},
    Email, Password, User,
};
use argon2::{
    password_hash::SaltString, Algorithm, Argon2, Params, PasswordHash, PasswordHasher,
    PasswordVerifier, Version,
};

use sqlx::MySqlPool;
use std::error::Error;

#[derive(Debug, Clone)]
pub struct MySqlUserStore {
    pub pool: MySqlPool,
}

impl MySqlUserStore {
    pub fn new(pool: MySqlPool) -> Self {
        Self { pool }
    }
}

#[async_trait::async_trait]
impl UserStore for MySqlUserStore {
    #[tracing::instrument(name = "Adding user to MySql", skip_all)]
    async fn add_user(&mut self, user: User) -> Result<(), UserStoreError> {
        let user_exist = self.get_user(user.email()).await;

        if user_exist.is_ok() {
            return Err(UserStoreError::UserAlreadyExists);
        }

        let password_hash = compute_password_hash(user.password().as_ref().to_string())
            .await
            .map_err(|_| UserStoreError::UnexpectedError)?;

        sqlx::query!(
            "
            INSERT INTO users (email, password_hash, requires_2fa) 
            VALUES (?, ?, ?)
            ",
            user.email().as_ref(),
            password_hash,
            user.has_2fa()
        )
        .execute(&self.pool)
        .await
        .map_err(|_| UserStoreError::UnexpectedError)?;

        Ok(())
    }

    #[tracing::instrument(name = "Retrieving user from MySql", skip_all)]
    async fn get_user(&self, email: &Email) -> Result<User, UserStoreError> {
        let record = sqlx::query!(
            r#"
            SELECT 
                email, 
                password_hash, 
                requires_2fa as "requires_2fa: bool"
            FROM 
                users
            WHERE
                email = ?
            "#,
            email.as_ref()
        )
        .fetch_one(&self.pool)
        .await
        .map_err(|_| UserStoreError::UserNotFound)?;

        let email = Email::parse(record.email).map_err(|_| UserStoreError::UnexpectedError)?;

        let password = Password::from(record.password_hash); // Keep From for Password since it's already hashed

        Ok(User::new(email, password, record.requires_2fa))
    }

    #[tracing::instrument(name = "Validating user credentials in MySql", skip_all)]
    async fn validate_user(
        &self,
        email: &Email,
        password: &Password,
    ) -> Result<(), UserStoreError> {
        let record = sqlx::query!(
            "
            SELECT 
                password_hash
            FROM 
                users
            WHERE
                email = ?
            ",
            email.as_ref()
        )
        .fetch_one(&self.pool)
        .await
        .map_err(|_| UserStoreError::UserNotFound)?;

        verify_password_hash(record.password_hash, password.as_ref().to_string())
            .await
            .map_err(|_| UserStoreError::IncorrectCredentials)
    }
}

#[tracing::instrument(name = "Verify password hash", skip_all)]
async fn verify_password_hash(
    expected_password_hash: String,
    password_candidate: String,
) -> Result<(), Box<dyn Error + Send + Sync>> {
    // The span represents the execution context for the compute_password_hash function.
    let current_span = tracing::Span::current();

    let result = tokio::task::spawn_blocking(move || {
        // ensures that the operations within the closure are executed within the context of the current span.
        // useful for tracing operations that are performed in a different thread or task, such as within tokio::task::spawn_blocking.
        current_span.in_scope(|| {
            let expected_password_hash: PasswordHash<'_> =
                PasswordHash::new(&expected_password_hash)?;

            Argon2::default()
                .verify_password(password_candidate.as_bytes(), &expected_password_hash)
                .map_err(|e| e.into())
        })
    })
    .await;

    result?
}

#[tracing::instrument(name = "Computing password hash", skip_all)]
async fn compute_password_hash(password: String) -> Result<String, Box<dyn Error + Send + Sync>> {

    let current_span: tracing::Span = tracing::Span::current(); 

    let result = tokio::task::spawn_blocking(move || {
        // This code block ensures that the operations within the closure are executed within the context of the current span. 
        // This is especially useful for tracing operations that are performed in a different thread or task, such as within tokio::task::spawn_blocking.
        current_span.in_scope(|| { 
            let salt: SaltString = SaltString::generate(&mut rand::thread_rng());
            let password_hash = Argon2::new(
                Algorithm::Argon2id,
                Version::V0x13,
                Params::new(15000, 2, 1, None)?,
            )
            .hash_password(password.as_bytes(), &salt)?
            .to_string();

            Ok(password_hash)
        })
    })
    .await;

    result?
}
