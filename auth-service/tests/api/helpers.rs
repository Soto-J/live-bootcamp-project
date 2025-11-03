use auth_service::{
    app_state::app_state::{AppState, BannedTokenStoreType, TwoFACodeStoreType},
    get_mysql_pool,
    services::{
        data_stores::{HashmapTwoFACodeStore, HashsetBannedTokenStore, MySqlUserStore},
        MockEmailClient,
    },
    utils::constants::{test, DATABASE_URL, MYSQL_SERVER_URL},
    Application,
};

use fake::{
    faker::internet::en::{self, SafeEmail},
    Fake,
};
use reqwest::cookie::Jar;
use sqlx::MySqlPool;
use std::{ops::Range, sync::Arc};
use tokio::sync::RwLock;
use uuid::Uuid;

#[derive(Clone)]
pub struct TestApp {
    pub address: String,
    pub cookie_jar: Arc<Jar>,
    pub http_client: reqwest::Client,
    pub two_fa_code_store: TwoFACodeStoreType,
    pub banned_token_store: BannedTokenStoreType,
    pub db_name: String,
}

impl TestApp {
    pub async fn new() -> Self {
        let (mysql_pool, db_name) = configure_mysql().await;

        let user_store = Arc::new(RwLock::new(MySqlUserStore::new(mysql_pool)));
        let banned_token_store = Arc::new(RwLock::new(HashsetBannedTokenStore::default()));
        let two_fa_code_store = Arc::new(RwLock::new(HashmapTwoFACodeStore::default()));
        let email_client = Arc::new(RwLock::new(MockEmailClient));

        let app_state = AppState::new(
            user_store.clone(),
            banned_token_store.clone(),
            two_fa_code_store.clone(),
            email_client.clone(),
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
            banned_token_store,
            two_fa_code_store,
            db_name,
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

    pub async fn post_verify_2fa<Body>(&self, body: &Body) -> reqwest::Response
    where
        Body: serde::Serialize,
    {
        self.http_client
            .post(&format!("{}/verify-2fa", &self.address))
            .json(body)
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

pub async fn configure_mysql() -> (MySqlPool, String) {
    // Creating a new database for each test case

    let mysql_conn_url = MYSQL_SERVER_URL.to_owned();
    let db_name = Uuid::new_v4().to_string();

    configure_database(&mysql_conn_url, &db_name).await;

    let mysql_conn_url_with_db = format!("{}/{}", mysql_conn_url, db_name);

    let mysql_pool = get_mysql_pool(&mysql_conn_url_with_db)
        .await
        .expect("Configure mysql: Failed to create MySql connection pool.");

    (mysql_pool, db_name)
}

pub async fn configure_database(db_conn_string: &str, db_name: &str) {
    let mysql_pool = get_mysql_pool(&db_conn_string)
        .await
        .expect("Configure Database: Failed to create MySql connection pool.");

    // Create new database
    sqlx::query(&format!(r#"CREATE DATABASE `{}`;"#, db_name))
        .execute(&mysql_pool)
        .await
        .expect("Configure Database: Failed to create database.");

    mysql_pool.close().await;

    // Connect to new database
    let mysql_conn_url_with_db = format!("{}/{}", db_conn_string, db_name);

    let mysql_pool = get_mysql_pool(&mysql_conn_url_with_db)
        .await
        .expect("Failed to create MySql connection pool.");

    // Run migrations against new database
    sqlx::migrate!()
        .run(&mysql_pool)
        .await
        .expect("Failed to migrate database.");
}

pub async fn drop_mysql_database(db_name: &str) {
    let mysql_conn_url = MYSQL_SERVER_URL.to_owned();

    let mysql_pool = get_mysql_pool(&mysql_conn_url)
        .await
        .expect("Failed to create MySql connection pool.");

    sqlx::query(&format!(r#"DROP DATABASE `{}`"#, db_name))
        .execute(&mysql_pool)
        .await
        .expect("Failed to drop database");

    mysql_pool.close().await
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
