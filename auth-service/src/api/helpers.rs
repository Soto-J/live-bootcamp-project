use crate::{
    app_state::app_state::AppState,
    configure_mysql,
    services::{
        data_stores::{HashmapTwoFACodeStore, HashsetBannedTokenStore, MySqlUserStore},
        MockEmailClient,
    },
    Application,
};

use fake::{faker::internet::en::SafeEmail, Fake};
use secrecy::Secret;
use std::sync::Arc;
use tokio::sync::RwLock;

pub struct TestApp {
    pub address: String,
    pub http_client: reqwest::Client,
}

impl TestApp {
    pub async fn new() -> Self {
        let mysql_pool = configure_mysql().await;

        let user_store = Arc::new(RwLock::new(MySqlUserStore::new(mysql_pool)));
        let banned_token_store = Arc::new(RwLock::new(HashsetBannedTokenStore::default()));
        let two_fa_store = Arc::new(RwLock::new(HashmapTwoFACodeStore::default()));
        let email_client = Arc::new(RwLock::new(MockEmailClient));

        let app_state = AppState::new(user_store, banned_token_store, two_fa_store, email_client);

        let address = "127.0.0.1:0";

        let app = Application::build(app_state, address)
            .await
            .expect("Failed to build app");

        let address = format!("http://{}", app.address.clone());

        // Run the auth service in a separate async task
        // to avoid blocking the main test thread.
        #[allow(clippy::let_underscore_future)]
        let _ = tokio::spawn(app.run());

        let http_client = reqwest::Client::new();

        TestApp {
            address,
            http_client,
        }
    }

    pub async fn get_root(&self) -> reqwest::Response {
        self.http_client
            .get(&format!("{}/", &self.address))
            .send()
            .await
            .expect("Failed to execute request.")
    }

    pub async fn post_signup<Body>(&self, body: &Body) -> reqwest::Response
    where
        Body: serde::Serialize,
    {
        self.http_client
            .post(&format!("{}/signup", &self.address))
            .json(body)
            .send()
            .await
            .expect("Failed to execute request.")
    }

    pub async fn login_root(&self) -> reqwest::Response {
        self.http_client
            .post(&format!("{}/login", &self.address))
            .send()
            .await
            .expect("Login failed.")
    }

    pub async fn logout_root(&self) -> reqwest::Response {
        self.http_client
            .post(&format!("{}/logout", &self.address))
            .send()
            .await
            .expect("Logout failed.")
    }

    pub async fn verify_2fa_root(&self) -> reqwest::Response {
        self.http_client
            .post(&format!("{}/verify-2fa", &self.address))
            .send()
            .await
            .expect("2fa failed.")
    }

    pub async fn verify_token_root(&self) -> reqwest::Response {
        self.http_client
            .post(&format!("{}/verify-token", &self.address))
            .send()
            .await
            .expect("Verify token failed.")
    }
}

pub fn get_random_email() -> Secret<String> {
    Secret::new(SafeEmail().fake())
}

pub fn get_random_password() -> Secret<String> {
    use rand::{distr::Alphanumeric, rng, Rng};

    let password: String = rng()
        .sample_iter(&Alphanumeric)
        .take(15)
        .map(char::from)
        .collect();

    Secret::new(password)
}

pub fn get_invalid_password() -> Secret<String> {
    use rand::{distr::Alphanumeric, rng, Rng};

    let password: String = rng()
        .sample_iter(&Alphanumeric)
        .take(7)
        .map(char::from)
        .collect();

    Secret::new(password)
}
