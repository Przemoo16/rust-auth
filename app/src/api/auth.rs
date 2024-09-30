use crate::{
    api::{
        constant::{
            EMAIL_IS_ALREADY_TAKEN_MESSAGE, EMAIL_MAX_LENGTH, EMAIL_TOO_LONG_MESSAGE,
            FIELD_REQUIRED_MESSAGE, HOME_ROUTE, INVALID_CREDENTIALS_MESSAGE, INVALID_EMAIL_MESSAGE,
            PASSWORD_MAX_LENGTH, PASSWORD_MIN_LENGTH, PASSWORD_MISMATCH_MESSAGE,
            PASSWORD_TOO_LONG_MESSAGE, PASSWORD_TOO_SHORT_MESSAGE, PROTECTED_ROUTE,
        },
        middleware::RenderOptions,
        response::{create_client_side_redirect, create_redirect_for_authenticated},
    },
    libs::{
        auth::{is_anonymous, AuthSession},
        validation::is_valid_email,
    },
    operations::auth::{
        sign_in, sign_out, sign_up, SigninData, SigninError, SignupData, SignupError,
    },
    state::AppState,
};
use askama_axum::Template;
use axum::{
    extract::{Extension, Query, State},
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post},
    Form, Router,
};
use axum_login::predicate_required;
use serde::Deserialize;
use tracing::error;

pub fn create_auth_router() -> Router<AppState> {
    Router::new()
        .route(
            "/signup",
            get(get_signup)
                .layer(predicate_required!(
                    is_anonymous,
                    create_redirect_for_authenticated()
                ))
                .post(post_signup),
        )
        .route(
            "/signin",
            get(get_signin)
                .layer(predicate_required!(
                    is_anonymous,
                    create_redirect_for_authenticated()
                ))
                .post(post_signin),
        )
        .route("/signout", post(post_signout))
}

#[derive(Template)]
#[template(path = "pages/signup/index.html")]
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

#[derive(PartialEq, Default)]
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
struct SignupPayload {
    email: String,
    password: String,
    confirm_password: String,
}

#[derive(Template)]
#[template(path = "pages/signup/form.html")]
struct SignupFormTemplate<'a> {
    form_data: SignupFormData<'a>,
}

async fn post_signup(
    State(state): State<AppState>,
    mut auth_session: AuthSession,
    Form(payload): Form<SignupPayload>,
) -> impl IntoResponse {
    if let Err(form_data) = validate_signup_payload(&payload) {
        let template = SignupFormTemplate { form_data };
        return (StatusCode::UNPROCESSABLE_ENTITY, template).into_response();
    }
    match sign_up(
        SignupData {
            email: &payload.email,
            password: payload.password,
        },
        &state.db,
        &mut auth_session,
    )
    .await
    {
        Ok(_) => create_client_side_redirect(StatusCode::CREATED, PROTECTED_ROUTE).into_response(),
        Err(e) => match e {
            SignupError::UserEmailAlreadyExistsError => {
                let form_data = SignupFormData {
                    values: SignupFormValues {
                        email: &payload.email,
                    },
                    errors: SignupFormErrors {
                        email: Some(EMAIL_IS_ALREADY_TAKEN_MESSAGE),
                        ..Default::default()
                    },
                    ..Default::default()
                };
                let template = SignupFormTemplate { form_data };
                (StatusCode::CONFLICT, template).into_response()
            }
            _ => {
                error!("Failed to sign up: {:?}", e);
                StatusCode::INTERNAL_SERVER_ERROR.into_response()
            }
        },
    }
}

fn validate_signup_payload(payload: &SignupPayload) -> Result<(), SignupFormData> {
    let mut focus = SignupFormField::default();
    let mut errors = SignupFormErrors::default();

    if payload.confirm_password.is_empty() {
        errors.confirm_password = Some(FIELD_REQUIRED_MESSAGE);
        focus = SignupFormField::ConfirmPassword;
    } else if payload.confirm_password != payload.password {
        errors.confirm_password = Some(PASSWORD_MISMATCH_MESSAGE);
        focus = SignupFormField::ConfirmPassword;
    }

    if payload.password.is_empty() {
        errors.password = Some(FIELD_REQUIRED_MESSAGE);
        focus = SignupFormField::Password;
    } else if payload.password.len() < PASSWORD_MIN_LENGTH {
        errors.password = Some(PASSWORD_TOO_SHORT_MESSAGE);
        focus = SignupFormField::Password;
    } else if payload.password.len() > PASSWORD_MAX_LENGTH {
        errors.password = Some(PASSWORD_TOO_LONG_MESSAGE);
        focus = SignupFormField::Password;
    }

    if payload.email.is_empty() {
        errors.email = Some(FIELD_REQUIRED_MESSAGE);
        focus = SignupFormField::Email;
    } else if payload.email.len() > EMAIL_MAX_LENGTH {
        errors.email = Some(EMAIL_TOO_LONG_MESSAGE);
        focus = SignupFormField::Email;
    } else if !is_valid_email(&payload.email) {
        errors.email = Some(INVALID_EMAIL_MESSAGE);
        focus = SignupFormField::Email;
    }

    if !errors.has_errors() {
        return Ok(());
    }
    Err(SignupFormData {
        focus,
        values: SignupFormValues {
            email: &payload.email,
        },
        errors,
    })
}

#[derive(Deserialize)]
struct SigninParams {
    next: Option<String>,
}

#[derive(Template)]
#[template(path = "pages/signin/index.html")]
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

#[derive(PartialEq, Default)]
enum SigninFormField {
    #[default]
    Email,
    Password,
}

#[derive(Default)]
struct SigninFormValues<'a> {
    email: &'a str,
    next: Option<&'a str>,
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

async fn get_signin(
    Extension(options): Extension<RenderOptions>,
    params: Query<SigninParams>,
) -> impl IntoResponse {
    SigninTemplate {
        options,
        form_data: SigninFormData {
            values: SigninFormValues {
                next: params.next.as_deref(),
                ..Default::default()
            },
            ..Default::default()
        },
    }
    .into_response()
}

#[derive(Deserialize)]
struct SigninPayload {
    email: String,
    password: String,
    next: Option<String>,
}

#[derive(Template)]
#[template(path = "pages/signin/form.html")]
struct SigninFormTemplate<'a> {
    form_data: SigninFormData<'a>,
}

async fn post_signin(
    mut auth_session: AuthSession,
    Form(payload): Form<SigninPayload>,
) -> impl IntoResponse {
    if let Err(form_data) = validate_signin_payload(&payload) {
        let template = SigninFormTemplate { form_data };
        return (StatusCode::UNPROCESSABLE_ENTITY, template).into_response();
    }

    match sign_in(
        SigninData {
            email: payload.email.clone(),
            password: payload.password,
        },
        &mut auth_session,
    )
    .await
    {
        Ok(_) => {
            let next_url = payload.next.as_deref().unwrap_or(PROTECTED_ROUTE);
            create_client_side_redirect(StatusCode::OK, next_url).into_response()
        }
        Err(e) => match e {
            SigninError::InvalidCredentialsError => {
                let template = SigninFormTemplate {
                    form_data: SigninFormData {
                        values: SigninFormValues {
                            email: &payload.email,
                            next: payload.next.as_deref(),
                        },
                        errors: SigninFormErrors {
                            general: Some(INVALID_CREDENTIALS_MESSAGE),
                            ..Default::default()
                        },
                        ..Default::default()
                    },
                };
                (StatusCode::UNAUTHORIZED, template).into_response()
            }
            _ => {
                error!("Failed to sign in: {:?}", e);
                StatusCode::INTERNAL_SERVER_ERROR.into_response()
            }
        },
    }
}

fn validate_signin_payload(payload: &SigninPayload) -> Result<(), SigninFormData> {
    let mut focus = SigninFormField::default();
    let mut errors = SigninFormErrors::default();

    if payload.password.is_empty() {
        errors.password = Some(FIELD_REQUIRED_MESSAGE);
        focus = SigninFormField::Password;
    }

    if payload.email.is_empty() {
        errors.email = Some(FIELD_REQUIRED_MESSAGE);
        focus = SigninFormField::Email;
    } else if !is_valid_email(&payload.email) {
        errors.email = Some(INVALID_EMAIL_MESSAGE);
        focus = SigninFormField::Email;
    }

    if !errors.has_errors() {
        return Ok(());
    }
    Err(SigninFormData {
        focus,
        values: SigninFormValues {
            email: &payload.email,
            next: payload.next.as_deref(),
        },
        errors,
    })
}

async fn post_signout(mut auth_session: AuthSession) -> impl IntoResponse {
    match sign_out(&mut auth_session).await {
        Ok(_) => create_client_side_redirect(StatusCode::NO_CONTENT, HOME_ROUTE).into_response(),
        Err(e) => {
            error!("Failed to sign out: {:?}", e);
            StatusCode::INTERNAL_SERVER_ERROR.into_response()
        }
    }
}
