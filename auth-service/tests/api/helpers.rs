use auth_service::{
    app_state::app_state::{AppState, BannedTokenStoreType, TwoFACodeStoreType, UserStoreType},
    services::{HashmapTwoFACodeStore, HashmapUserStore, HashsetBannedTokenStore},
    utils::constants::test,
    Application,
};
use fake::{
    faker::internet::en::{self, SafeEmail},
    Fake,
};
use reqwest::cookie::Jar;
use std::{ops::Range, sync::Arc};
use tokio::sync::RwLock;

#[derive(Clone)]
pub struct TestApp {
    pub address: String,
    pub cookie_jar: Arc<Jar>,
    pub http_client: reqwest::Client,
    pub user_store: UserStoreType,
    pub two_fa_code_store: TwoFACodeStoreType,
    pub banned_token_store: BannedTokenStoreType,
}

impl TestApp {
    pub async fn new() -> Self {
        let user_store = Arc::new(RwLock::new(HashmapUserStore::default()));
        let banned_token_store = Arc::new(RwLock::new(HashsetBannedTokenStore::default()));
        let two_fa_code_store = Arc::new(RwLock::new(HashmapTwoFACodeStore::default()));

        let app_state = AppState::new(
            user_store.clone(),
            banned_token_store.clone(),
            two_fa_code_store.clone(),
        );

        let app = Application::build(app_state, test::APP_ADDRESS)
            .await
            .expect("Failed to build app");

        let address = format!("http://{}", app.address.clone());

        // Run the auth service in a separate async task
        // to avoid blocking the main test thread.
        #[allow(clippy::let_underscore_future)]
        let _ = tokio::spawn(app.run());

        let cookie_jar = Arc::new(Jar::default());
        let http_client = reqwest::Client::builder()
            .cookie_provider(cookie_jar.clone())
            .build()
            .unwrap();

        TestApp {
            address,
            cookie_jar,
            http_client,
            user_store,
            banned_token_store,
            two_fa_code_store,
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

    pub async fn post_login<Body>(&self, body: &Body) -> reqwest::Response
    where
        Body: serde::Serialize,
    {
        self.http_client
            .post(&format!("{}/login", &self.address))
            .json(body)
            .send()
            .await
            .expect("Failed to execute request.")
    }

    pub async fn post_logout(&self) -> reqwest::Response {
        self.http_client
            .post(&format!("{}/logout", &self.address))
            .send()
            .await
            .expect("Failed to execute request.")
    }

    pub async fn verify_2fa_root(&self) -> reqwest::Response {
        self.http_client
            .post(&format!("{}/verify-2fa", &self.address))
            .send()
            .await
            .expect("Failed to execute request.")
    }

    pub async fn post_verify_token<Body>(&self, body: &Body) -> reqwest::Response
    where
        Body: serde::Serialize,
    {
        self.http_client
            .post(&format!("{}/verify-token", &self.address))
            .json(body)
            .send()
            .await
            .expect("Failed to execute request.")
    }
}

pub fn get_random_email() -> String {
    SafeEmail().fake()
}

pub fn get_random_password() -> String {
    en::Password(Range { start: 8, end: 15 }).fake()
}

pub fn get_invalid_password() -> String {
    en::Password(Range { start: 0, end: 7 }).fake()
}
