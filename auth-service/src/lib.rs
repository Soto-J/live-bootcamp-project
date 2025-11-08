use crate::{
    app_state::app_state::AppState,
    routes::{
        login::login_handler, logout::logout_handler, signup::signup_handler,
        verify_2fa::verify_2fa_handler, verify_token::verify_token_handler,
    },
    utils::{
        constants::{DATABASE_URL, REDIS_HOST_NAME},
        tracing::{make_span_with_request_id, on_request, on_response},
    },
};

use axum::{routing::post, serve::Serve, Router};
use redis::{Client, RedisResult};
use reqwest::Method;
use sqlx::{mysql::MySqlPoolOptions, MySqlPool};
use std::{error::Error, io};
use tower_http::{cors::CorsLayer, services::ServeDir, trace::TraceLayer};

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
            .layer(
                TraceLayer::new_for_http()
                    .make_span_with(make_span_with_request_id)
                    .on_request(on_request)
                    .on_response(on_response),
            )
            .layer(cors);

        let listener = tokio::net::TcpListener::bind(address).await?;

        let address = listener.local_addr()?.to_string();
        let server = axum::serve(listener, router);

        Ok(Application { server, address })
    }

    pub async fn run(self) -> Result<(), io::Error> {
        tracing::info!("Listening on {}...", &self.address);
        self.server.await
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

pub fn configure_redis() -> redis::Connection {
    get_redis_client(REDIS_HOST_NAME.to_owned())
        .expect("Failed to get Rediss client")
        .get_connection()
        .expect("Failed to get Redis connection")
}

pub fn get_redis_client(redis_hostname: String) -> RedisResult<Client> {
    let redis_url = format!("redis://{}/", redis_hostname);
    redis::Client::open(redis_url)
}
