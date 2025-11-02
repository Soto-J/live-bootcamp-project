use auth_service::{
    Application, app_state::app_state::AppState, get_mysql_pool, services::{HashmapTwoFACodeStore, HashmapUserStore, HashsetBannedTokenStore, MockEmailClient}, utils::constants::{DATABASE_URL, prod}
};
use sqlx::{MySql, Pool};
use std::sync::Arc;
use tokio::sync::RwLock;

#[tokio::main]
async fn main() {
    let mysql_pool = configure_mysql().await;

    let user_store = Arc::new(RwLock::new(HashmapUserStore::default()));
    let banned_token_store = Arc::new(RwLock::new(HashsetBannedTokenStore::default()));
    let two_fa_store = Arc::new(RwLock::new(HashmapTwoFACodeStore::default()));
    let email_client = Arc::new(RwLock::new(MockEmailClient));

    let app_state = AppState::new(user_store, banned_token_store, two_fa_store, email_client);

    let app = Application::build(app_state, prod::APP_ADDRESS)
        .await
        .expect("Failed to build app.");

    app.run().await.expect("Failed to run app.")
}



async fn configure_mysql() -> Pool<MySql> {
    // Create a new database connection pool
    let mysql_pool = get_mysql_pool(&DATABASE_URL)
        .await
        .expect("Failed to create MySql connection pool!");

    // Run database migrations 
    sqlx::migrate!()
        .run(&mysql_pool)
        .await
        .expect("Failed to run migrations");

    mysql_pool
}