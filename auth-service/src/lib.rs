use crate::{
    app_state::app_state::AppState,
    domain::error::AuthAPIError,
    routes::{
        login::login_handler, logout::logout_handler, signup::signup_handler,
        verify_2fa::verify_2fa_handler, verify_token::verify_token_handler,
    },
    utils::constants::DATABASE_URL,
};

use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    routing::post,
    serve::Serve,
    Json, Router,
};
use reqwest::Method;
use serde::{Deserialize, Serialize};
use sqlx::{mysql::MySqlPoolOptions, MySqlPool};
use std::error::Error;
use tower_http::{cors::CorsLayer, services::ServeDir};

pub mod api;
pub mod app_state;
pub mod domain;
pub mod routes;
pub mod services;
pub mod utils;

pub struct Application {
    server: Serve<Router, Router>,
    pub address: String,
}

impl Application {
    pub async fn build(app_state: AppState, address: &str) -> Result<Self, Box<dyn Error>> {
        let allowed_origins = [
            "http://localhost:8000".parse()?,
            "http://[droplet_IP]:8000".parse()?,
        ];

        let cors = CorsLayer::new()
            .allow_methods([Method::GET, Method::POST])
            .allow_credentials(true)
            .allow_origin(allowed_origins);

        let router = Router::new()
            .nest_service("/", ServeDir::new("assets"))
            .route("/signup", post(signup_handler))
            .route("/login", post(login_handler))
            .route("/logout", post(logout_handler))
            .route("/verify-2fa", post(verify_2fa_handler))
            .route("/verify-token", post(verify_token_handler))
            .with_state(app_state)
            .layer(cors);

        let listener = tokio::net::TcpListener::bind(address).await?;

        let address = listener.local_addr()?.to_string();
        let server = axum::serve(listener, router);

        Ok(Application { server, address })
    }

    pub async fn run(self) -> Result<(), std::io::Error> {
        println!("Listening on {}...", &self.address);
        self.server.await
    }
}

#[derive(Serialize, Deserialize, Clone)]
pub struct ErrorResponse {
    pub error: String,
}

impl IntoResponse for AuthAPIError {
    fn into_response(self) -> Response {
        let (status, error_message) = match self {
            AuthAPIError::UserAlreadyExists => (StatusCode::CONFLICT, "User already exists"),
            AuthAPIError::InvalidCredentials => (StatusCode::BAD_REQUEST, "Invalid credentials"),
            AuthAPIError::IncorrectCredentials => {
                (StatusCode::UNAUTHORIZED, "Incorrect credentials")
            }
            AuthAPIError::MissingToken => (StatusCode::BAD_REQUEST, "Missing auth token"),
            AuthAPIError::InvalidToken => (StatusCode::UNAUTHORIZED, "Invalid auth token"),
            AuthAPIError::Missing2FA => (StatusCode::PARTIAL_CONTENT, "2FA Missing"),
            AuthAPIError::UnexpectedError => {
                (StatusCode::INTERNAL_SERVER_ERROR, "Unexpected error")
            }
        };

        let body = Json(ErrorResponse {
            error: error_message.to_string(),
        });

        (status, body).into_response()
    }
}

pub async fn configure_mysql() -> MySqlPool {
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

pub async fn get_mysql_pool(url: &str) -> Result<MySqlPool, sqlx::Error> {
    MySqlPoolOptions::new()
        .max_connections(5)
        .connect(url)
        .await
}
