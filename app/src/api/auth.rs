use crate::api::middlewares::RenderOptions;
use crate::constants::auth::{
    EMAIL_IS_ALREADY_TAKEN_MESSAGE, EMAIL_MAX_LENGTH, EMAIL_TOO_LONG_MESSAGE,
    FIELD_REQUIRED_MESSAGE, INVALID_EMAIL_MESSAGE, PASSWORD_MAX_LENGTH, PASSWORD_MIN_LENGTH,
    PASSWORD_MISMATCH_MESSAGE, PASSWORD_TOO_LONG_MESSAGE, PASSWORD_TOO_SHORT_MESSAGE,
};
use crate::libs::validation::is_valid_email;
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

pub fn create_auth_router() -> Router<AppState> {
    Router::new().route("/signup", get(get_signup).post(post_signup))
}

struct SignupFormData<'a> {
    focus: SignupFormField,
    values: SignupFormValues<'a>,
    errors: SignupFormErrors<'a>,
}

impl Default for SignupFormData<'_> {
    fn default() -> Self {
        Self {
            focus: SignupFormField::Email,
            values: SignupFormValues::default(),
            errors: SignupFormErrors::default(),
        }
    }
}

enum SignupFormField {
    Email,
    Password,
    ConfirmPassword,
}

struct SignupFormValues<'a> {
    email: &'a str,
}

impl Default for SignupFormValues<'_> {
    fn default() -> Self {
        Self { email: "" }
    }
}

struct SignupFormErrors<'a> {
    email: Vec<&'a str>,
    password: Vec<&'a str>,
    confirm_password: Vec<&'a str>,
}

impl Default for SignupFormErrors<'_> {
    fn default() -> Self {
        Self {
            email: vec![],
            password: vec![],
            confirm_password: vec![],
        }
    }
}

impl SignupFormErrors<'_> {
    fn has_errors(&self) -> bool {
        !self.email.is_empty() || !self.password.is_empty() || !self.confirm_password.is_empty()
    }
}

#[derive(Template)]
#[template(path = "signup/index.html")]
struct SignupTemplate<'a> {
    options: RenderOptions,
    form_data: SignupFormData<'a>,
}

#[derive(Template)]
#[template(path = "signup/form.html")]
struct SignupFormTemplate<'a> {
    form_data: SignupFormData<'a>,
}

async fn get_signup(Extension(options): Extension<RenderOptions>) -> SignupTemplate<'static> {
    SignupTemplate {
        options,
        form_data: SignupFormData::default(),
    }
}

#[derive(Deserialize)]
struct SignupRequest {
    email: String,
    password: String,
    confirm_password: String,
}

async fn post_signup(
    State(state): State<AppState>,
    Form(data): Form<SignupRequest>,
) -> impl IntoResponse {
    if let Err(form_data) = validate_signup_request(&data) {
        let template = SignupFormTemplate { form_data };
        return (StatusCode::UNPROCESSABLE_ENTITY, template).into_response();
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
            SignupError::UserEmailAlreadyExistsError => {
                let mut form_data = SignupFormData::default();
                let mut errors = SignupFormErrors::default();
                errors.email.push(EMAIL_IS_ALREADY_TAKEN_MESSAGE);
                form_data.errors = errors;
                let template = SignupFormTemplate { form_data };
                (StatusCode::CONFLICT, template).into_response()
            }
            _ => {
                error!("Failed to signup: {:?}", e);
                StatusCode::INTERNAL_SERVER_ERROR.into_response()
            }
        },
    }
}

fn validate_signup_request(data: &SignupRequest) -> Result<(), SignupFormData> {
    let mut focus = SignupFormField::Email;
    let mut errors = SignupFormErrors::default();
    if data.password != data.confirm_password {
        errors.confirm_password.push(PASSWORD_MISMATCH_MESSAGE);
        focus = SignupFormField::ConfirmPassword;
    }
    if data.password.len() < PASSWORD_MIN_LENGTH {
        errors.password.push(PASSWORD_TOO_SHORT_MESSAGE);
        focus = SignupFormField::Password;
    }
    if data.password.len() > PASSWORD_MAX_LENGTH {
        errors.password.push(PASSWORD_TOO_LONG_MESSAGE);
        focus = SignupFormField::Password;
    }
    // TODO: Add email validation
    if data.email.is_empty() {
        errors.email.push(FIELD_REQUIRED_MESSAGE);
        focus = SignupFormField::Email;
    }
    if data.email.len() > EMAIL_MAX_LENGTH {
        errors.email.push(EMAIL_TOO_LONG_MESSAGE);
        focus = SignupFormField::Email;
    }
    if !is_valid_email(&data.email) {
        errors.email.push(INVALID_EMAIL_MESSAGE);
        focus = SignupFormField::Email;
    }
    if !errors.has_errors() {
        return Ok(());
    }
    return Err(SignupFormData {
        focus,
        values: SignupFormValues { email: &data.email },
        errors,
    });
}
