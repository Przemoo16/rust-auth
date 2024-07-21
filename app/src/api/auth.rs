use crate::api::middlewares::RenderOptions;
use crate::operations::auth::{signup, SignupData, SignupError};
use crate::state::AppState;
use askama_axum::Template;
use axum::{
    extract::{Extension, State},
    http::StatusCode,
    response::IntoResponse,
    routing::get,
    Form, Router,
};
use serde::Deserialize;
use tracing::error;
use validator::Validate;

pub fn create_auth_router() -> Router<AppState> {
    Router::new().route("/signup", get(get_signup).post(post_signup))
}

#[derive(Template)]
#[template(path = "signup/index.html")]
struct SignupTemplate {
    options: RenderOptions,
}

async fn get_signup(Extension(options): Extension<RenderOptions>) -> SignupTemplate {
    SignupTemplate { options }
}

#[derive(Validate, Deserialize)]
struct SignupRequest {
    #[validate(email)]
    #[validate(length(max = 254))]
    email: String,
    #[validate(length(min = 8, max = 64))]
    password: String,
    #[validate(must_match(other = "password"))]
    confirm_password: String,
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
            SignupError::UserEmailAlreadyExistsError => StatusCode::CONFLICT.into_response(),
            _ => {
                error!("Failed to signup: {:?}", e);
                StatusCode::INTERNAL_SERVER_ERROR.into_response()
            }
        },
    }
}
