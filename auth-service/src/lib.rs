use crate::app_state::AppState;

use axum::{routing::post, serve::Serve, Router};
use std::error::Error;
use tower_http::services::ServeDir;

pub mod app_state;
pub mod routes;
pub mod services;
mod api;
mod domain;

pub struct Application {
    server: Serve<Router, Router>,
    pub address: String,
}

impl Application {
    pub async fn build(app_state: AppState, address: &str) -> Result<Self, Box<dyn Error>> {
        let router = Router::new()
            .nest_service("/", ServeDir::new("assets"))
            .route("/signup", post(routes::signup))
            .route("/login", post(routes::login_handler))
            .route("/logout", post(routes::logout_handler))
            .route("/verify-2fa", post(routes::verify_2fa_handler))
            .route("/verify-token", post(routes::verify_token_handler))
            .with_state(app_state);

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
