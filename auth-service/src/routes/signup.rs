use {
    crate::{
        app_state::AppState,
        domain::{AuthAPIError, User},
    },
    axum::{extract::State, http::StatusCode, response::IntoResponse, Json},
    serde::{Deserialize, Serialize},
};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SignupRequest {
    pub email: String,
    pub password: String,
    #[serde(rename = "requires2FA")]
    pub requires_2fa: bool,
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct SignupResponse {
    pub message: String,
}

pub async fn signup(
    State(state): State<AppState>,
    Json(request): Json<SignupRequest>,
) -> Result<impl IntoResponse, AuthAPIError> {
    let is_invalid =
        request.email.is_empty() || !request.email.contains('@') || request.password.len() < 8;

    if is_invalid {
        return Err(AuthAPIError::InvalidCredentials);
    }

    let user = User::new(request.email, request.password, request.requires_2fa);
    let mut user_store = state.user_store.write().await;

    if let Err(_) = user_store.add_user(user) {
        return Err(AuthAPIError::UserAlreadyExists);
    };

    Ok((
        StatusCode::CREATED,
        Json(SignupResponse {
            message: "User created successfully!".to_string(),
        }),
    ))
}

pub async fn signup_malformed_request_422() -> impl IntoResponse {
    StatusCode::UNPROCESSABLE_ENTITY.into_response()
}
