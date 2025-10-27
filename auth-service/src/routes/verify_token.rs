use crate::{
    app_state::app_state::AppState, domain::error::AuthAPIError, utils::auth::validate_token,
};

use axum::{extract::State, response::IntoResponse, Json};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct VerifyTokenRequest {
    token: String,
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct VerifyTokenResponse {
    message: String,
}

pub async fn verify_token_handler(
    State(state): State<AppState>,
    Json(request): Json<VerifyTokenRequest>,
) -> impl IntoResponse {
    if validate_token(&request.token, state.banned_token_store)
        .await
        .is_err()
    {
        return Err(AuthAPIError::InvalidToken);
    }

    Ok(())
}
