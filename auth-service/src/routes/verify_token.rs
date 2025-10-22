use crate::{domain::AuthAPIError, utils::validate_token};

use axum::{response::IntoResponse, Json};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct VerifyTokenRequest {
    token: String,
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct VerifyTokenResponse {
    message: String,
}

pub async fn verify_token_handler(Json(request): Json<VerifyTokenRequest>) -> impl IntoResponse {
    match validate_token(&request.token).await {
        Ok(_) => Ok(()),
        Err(_) => Err(AuthAPIError::InvalidCredentials),
    }
}
