use crate::operations::auth::{signup, SignupData, SignupError};
use crate::state::AppState;
use axum::{extract::State, http::StatusCode, response::IntoResponse, routing::post, Form, Router};
use serde::Deserialize;
use validator::Validate;

pub fn create_auth_router() -> Router<AppState> {
    Router::new().route("/signup", post(post_signup))
}

#[derive(Validate, Deserialize)]
struct SignupRequest {
    #[validate(email)]
    #[validate(length(max = 254))]
    email: String,
    #[validate(length(min = 8, max = 64))]
    password: String,
    #[validate(must_match(other = "password"))]
    #[serde(rename = "confirmed-password")]
    confirmed_password: String,
}

async fn post_signup(
    State(state): State<AppState>,
    Form(data): Form<SignupRequest>,
) -> impl IntoResponse {
    if let Err(_) = data.validate() {
        return StatusCode::UNPROCESSABLE_ENTITY.into_response();
    }
    match signup(
        SignupData {
            email: &data.email,
            password: data.password,
        },
        &state.db,
    )
    .await
    {
        Ok(_) => StatusCode::CREATED.into_response(),
        Err(e) => match e {
            SignupError::UserAlreadyExistsError => StatusCode::CONFLICT.into_response(),
            // TODO: Log error
            _ => StatusCode::INTERNAL_SERVER_ERROR.into_response(),
        },
    }
}
