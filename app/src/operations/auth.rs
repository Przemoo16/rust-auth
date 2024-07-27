use crate::db::connection::Database;
use crate::db::user::{create_user, CreateUserData, CreateUserError};
use crate::libs::password::{hash_password_in_separate_thread, HashPasswordError};
use std::error::Error;
use std::fmt::{Display, Formatter, Result as FormatResult};

#[derive(Debug)]
pub enum SignupError {
    HashPasswordError(HashPasswordError),
    UserEmailAlreadyExistsError,
    CreateUserError(CreateUserError),
}

impl Error for SignupError {}

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

impl Display for SignupError {
    fn fmt(&self, f: &mut Formatter<'_>) -> FormatResult {
        match self {
            SignupError::HashPasswordError(e) => write!(f, "Hash password error: {}", e),
            SignupError::UserEmailAlreadyExistsError => write!(f, "User email already exists"),
            SignupError::CreateUserError(e) => write!(f, "Create user error: {}", e),
        }
    }
}

pub struct SignupData<'a> {
    pub email: &'a str,
    pub password: String,
}

pub async fn signup(data: SignupData<'_>, db: &Database) -> Result<(), SignupError> {
    let hashed_password = hash_password_in_separate_thread(data.password).await?;
    create_user(
        CreateUserData {
            email: data.email,
            password: &hashed_password,
        },
        db,
    )
    .await?;
    // TODO: Login here
    Ok(())
}
