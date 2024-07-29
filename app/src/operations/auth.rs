use crate::db::connection::Database;
use crate::db::user::{create_user, CreateUserData, CreateUserError};
use crate::libs::auth::{AuthError, AuthSession, Credentials};
use crate::libs::password::{hash_password_in_separate_thread, HashPasswordError};
use std::error::Error;
use std::fmt::{Display, Formatter, Result as FormatResult};

pub struct SignupData<'a> {
    pub email: &'a str,
    pub password: String,
}

#[derive(Debug)]
pub enum SignupError {
    HashPasswordError(HashPasswordError),
    UserEmailAlreadyExistsError,
    CreateUserError(CreateUserError),
    LoginError(AuthError),
}

impl Error for SignupError {}

impl Display for SignupError {
    fn fmt(&self, f: &mut Formatter<'_>) -> FormatResult {
        match self {
            SignupError::HashPasswordError(e) => write!(f, "Hash password error: {}", e),
            SignupError::UserEmailAlreadyExistsError => write!(f, "User email already exists"),
            SignupError::CreateUserError(e) => write!(f, "Create user error: {}", e),
            SignupError::LoginError(e) => write!(f, "Login error: {}", e),
        }
    }
}

impl From<HashPasswordError> for SignupError {
    fn from(value: HashPasswordError) -> Self {
        SignupError::HashPasswordError(value)
    }
}

impl From<CreateUserError> for SignupError {
    fn from(value: CreateUserError) -> Self {
        match value {
            CreateUserError::EmailAlreadyExistsError => SignupError::UserEmailAlreadyExistsError,
            other => SignupError::CreateUserError(other),
        }
    }
}

impl From<AuthError> for SignupError {
    fn from(value: AuthError) -> Self {
        SignupError::LoginError(value)
    }
}

pub async fn signup(
    data: SignupData<'_>,
    db: &Database,
    auth_session: &mut AuthSession,
) -> Result<(), SignupError> {
    let hashed_password = hash_password_in_separate_thread(data.password).await?;
    let user = create_user(
        CreateUserData {
            email: data.email,
            password: &hashed_password,
        },
        db,
    )
    .await?;
    auth_session.login(&user).await?;
    Ok(())
}

pub struct SigninData {
    pub email: String,
    pub password: String,
}

#[derive(Debug)]
pub enum SigninError {
    InvalidCredentialsError,
    AuthenticationError(AuthError),
}

impl Error for SigninError {}

impl Display for SigninError {
    fn fmt(&self, f: &mut Formatter<'_>) -> FormatResult {
        match self {
            SigninError::InvalidCredentialsError => write!(f, "Invalid credentials"),
            SigninError::AuthenticationError(e) => write!(f, "Authentication error: {}", e),
        }
    }
}

impl From<AuthError> for SigninError {
    fn from(value: AuthError) -> Self {
        SigninError::AuthenticationError(value)
    }
}

pub async fn signin(data: SigninData, auth_session: &mut AuthSession) -> Result<(), SigninError> {
    let user = auth_session
        .authenticate(Credentials {
            email: data.email,
            password: data.password,
        })
        .await?
        .ok_or(SigninError::InvalidCredentialsError)?;
    auth_session.login(&user).await?;
    Ok(())
}

#[derive(Debug)]
pub struct LogoutError(AuthError);

impl Error for LogoutError {}

impl Display for LogoutError {
    fn fmt(&self, f: &mut Formatter<'_>) -> FormatResult {
        write!(f, "Logout error: {}", self.0)
    }
}

impl From<AuthError> for LogoutError {
    fn from(value: AuthError) -> Self {
        LogoutError(value)
    }
}

pub async fn logout(auth_session: &mut AuthSession) -> Result<(), LogoutError> {
    auth_session.logout().await?;
    Ok(())
}
