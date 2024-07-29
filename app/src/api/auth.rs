use crate::api::middleware::RenderOptions;
use crate::constants::auth::{
    EMAIL_IS_ALREADY_TAKEN_MESSAGE, EMAIL_MAX_LENGTH, EMAIL_TOO_LONG_MESSAGE,
    FIELD_REQUIRED_MESSAGE, INVALID_CREDENTIALS_MESSAGE, INVALID_EMAIL_MESSAGE,
    PASSWORD_MAX_LENGTH, PASSWORD_MIN_LENGTH, PASSWORD_MISMATCH_MESSAGE, PASSWORD_TOO_LONG_MESSAGE,
    PASSWORD_TOO_SHORT_MESSAGE,
};
use crate::libs::auth::AuthSession;
use crate::libs::validation::is_valid_email;
use crate::operations::auth::{
    logout, signin, signup, SigninData, SigninError, SignupData, SignupError,
};
use crate::state::AppState;
use askama_axum::Template;
use axum::{
    extract::{Extension, State},
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post},
    Form, Router,
};
use serde::Deserialize;
use tracing::error;

pub fn create_auth_router() -> Router<AppState> {
    Router::new()
        .route("/signup", get(get_signup).post(post_signup))
        .route("/signin", get(get_signin).post(post_signin))
        .route("/logout", post(post_logout))
}

#[derive(Template)]
#[template(path = "signup/index.html")]
struct SignupTemplate<'a> {
    options: RenderOptions,
    form_data: SignupFormData<'a>,
}

#[derive(Default)]
struct SignupFormData<'a> {
    focus: SignupFormField,
    values: SignupFormValues<'a>,
    errors: SignupFormErrors<'a>,
}

#[derive(Default)]
enum SignupFormField {
    #[default]
    Email,
    Password,
    ConfirmPassword,
}

#[derive(Default)]
struct SignupFormValues<'a> {
    email: &'a str,
}

#[derive(Default)]
struct SignupFormErrors<'a> {
    email: Option<&'a str>,
    password: Option<&'a str>,
    confirm_password: Option<&'a str>,
}

impl SignupFormErrors<'_> {
    fn has_errors(&self) -> bool {
        self.email.is_some() || self.password.is_some() || self.confirm_password.is_some()
    }
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

#[derive(Template)]
#[template(path = "signup/form.html")]
struct SignupFormTemplate<'a> {
    form_data: SignupFormData<'a>,
}

async fn post_signup(
    State(state): State<AppState>,
    mut auth_session: AuthSession,
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
        &mut auth_session,
    )
    .await
    {
        Ok(_) => StatusCode::CREATED.into_response(), // TODO: Redirect
        Err(e) => match e {
            SignupError::UserEmailAlreadyExistsError => {
                let mut errors = SignupFormErrors::default();
                errors.email = Some(EMAIL_IS_ALREADY_TAKEN_MESSAGE);
                let form_data = SignupFormData {
                    errors,
                    ..Default::default()
                };
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
    let mut focus = SignupFormField::default();
    let mut errors = SignupFormErrors::default();

    if data.confirm_password.is_empty() {
        errors.confirm_password = Some(FIELD_REQUIRED_MESSAGE);
        focus = SignupFormField::ConfirmPassword;
    } else if data.password != data.confirm_password {
        errors.confirm_password = Some(PASSWORD_MISMATCH_MESSAGE);
        focus = SignupFormField::ConfirmPassword;
    }

    if data.password.is_empty() {
        errors.password = Some(FIELD_REQUIRED_MESSAGE);
        focus = SignupFormField::Password;
    } else if data.password.len() < PASSWORD_MIN_LENGTH {
        errors.password = Some(PASSWORD_TOO_SHORT_MESSAGE);
        focus = SignupFormField::Password;
    } else if data.password.len() > PASSWORD_MAX_LENGTH {
        errors.password = Some(PASSWORD_TOO_LONG_MESSAGE);
        focus = SignupFormField::Password;
    }

    if data.email.is_empty() {
        errors.email = Some(FIELD_REQUIRED_MESSAGE);
        focus = SignupFormField::Email;
    } else if data.email.len() > EMAIL_MAX_LENGTH {
        errors.email = Some(EMAIL_TOO_LONG_MESSAGE);
        focus = SignupFormField::Email;
    } else if !is_valid_email(&data.email) {
        errors.email = Some(INVALID_EMAIL_MESSAGE);
        focus = SignupFormField::Email;
    }

    if !errors.has_errors() {
        return Ok(());
    }
    Err(SignupFormData {
        focus,
        values: SignupFormValues { email: &data.email },
        errors,
    })
}

#[derive(Template)]
#[template(path = "signin/index.html")]
struct SigninTemplate<'a> {
    options: RenderOptions,
    form_data: SigninFormData<'a>,
}

#[derive(Default)]
struct SigninFormData<'a> {
    focus: SigninFormField,
    values: SigninFormValues<'a>,
    errors: SigninFormErrors<'a>,
}

#[derive(Default)]
enum SigninFormField {
    #[default]
    Email,
    Password,
}

#[derive(Default)]
struct SigninFormValues<'a> {
    email: &'a str,
}

#[derive(Default)]
struct SigninFormErrors<'a> {
    email: Option<&'a str>,
    password: Option<&'a str>,
    general: Option<&'a str>,
}

impl SigninFormErrors<'_> {
    fn has_errors(&self) -> bool {
        self.email.is_some() || self.password.is_some() || self.general.is_some()
    }
}

async fn get_signin(Extension(options): Extension<RenderOptions>) -> SigninTemplate<'static> {
    SigninTemplate {
        options,
        form_data: SigninFormData::default(),
    }
}

#[derive(Deserialize)]
struct SigninRequest {
    email: String,
    password: String,
    // TODO: Add next URL
}

#[derive(Template)]
#[template(path = "signin/form.html")]
struct SigninFormTemplate<'a> {
    form_data: SigninFormData<'a>,
}

async fn post_signin(
    mut auth_session: AuthSession,
    Form(data): Form<SigninRequest>,
) -> impl IntoResponse {
    if let Err(form_data) = validate_signin_request(&data) {
        let template = SigninFormTemplate { form_data };
        return (StatusCode::UNPROCESSABLE_ENTITY, template).into_response();
    }

    match signin(
        SigninData {
            email: data.email,
            password: data.password,
        },
        &mut auth_session,
    )
    .await
    {
        Ok(_) => StatusCode::OK.into_response(), // TODO: Redirect
        Err(e) => match e {
            SigninError::InvalidCredentialsError => {
                let mut errors = SigninFormErrors::default();
                errors.general = Some(INVALID_CREDENTIALS_MESSAGE);
                let form_data = SigninFormData {
                    errors,
                    ..Default::default()
                };
                let template = SigninFormTemplate { form_data };
                (StatusCode::UNAUTHORIZED, template).into_response()
            }
            _ => {
                error!("Failed to signin: {:?}", e);
                StatusCode::INTERNAL_SERVER_ERROR.into_response()
            }
        },
    }
}

fn validate_signin_request(data: &SigninRequest) -> Result<(), SigninFormData> {
    let mut focus = SigninFormField::default();
    let mut errors = SigninFormErrors::default();

    if data.password.is_empty() {
        errors.password = Some(FIELD_REQUIRED_MESSAGE);
        focus = SigninFormField::Password;
    }

    if data.email.is_empty() {
        errors.email = Some(FIELD_REQUIRED_MESSAGE);
        focus = SigninFormField::Email;
    } else if !is_valid_email(&data.email) {
        errors.email = Some(INVALID_EMAIL_MESSAGE);
        focus = SigninFormField::Email;
    }

    if !errors.has_errors() {
        return Ok(());
    }
    Err(SigninFormData {
        focus,
        values: SigninFormValues { email: &data.email },
        errors,
    })
}

async fn post_logout(mut auth_session: AuthSession) -> impl IntoResponse {
    match logout(&mut auth_session).await {
        Ok(_) => StatusCode::OK.into_response(), // TODO: Redirect
        Err(e) => {
            error!("Failed to logout: {:?}", e);
            StatusCode::INTERNAL_SERVER_ERROR.into_response()
        }
    }
}
