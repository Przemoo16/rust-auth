use crate::db::connection::Database;
use sqlx::{query_as, Error as SqlxError};
use std::{
    error::Error,
    fmt::{Debug as FormatDebug, Display, Formatter, Result as FormatResult},
};

#[derive(Clone)]
pub struct AuthUser {
    pub id: i32,
    pub password: String,
}

impl FormatDebug for AuthUser {
    fn fmt(&self, f: &mut Formatter<'_>) -> FormatResult {
        f.debug_struct("AuthUser")
            .field("id", &self.id)
            .field("password", &"********")
            .finish()
    }
}

pub struct CreateUserData<'a> {
    pub email: &'a str,
    pub password: &'a str,
}

#[derive(Debug)]
pub enum CreateUserError {
    EmailAlreadyExistsError,
    DatabaseError(SqlxError),
}

impl Error for CreateUserError {}

impl Display for CreateUserError {
    fn fmt(&self, f: &mut Formatter<'_>) -> FormatResult {
        match self {
            CreateUserError::EmailAlreadyExistsError => {
                write!(f, "Email already exists")
            }
            CreateUserError::DatabaseError(e) => {
                write!(f, "Database error: {}", e)
            }
        }
    }
}

impl From<SqlxError> for CreateUserError {
    fn from(value: SqlxError) -> Self {
        match value.as_database_error() {
            Some(e) if e.is_unique_violation() => CreateUserError::EmailAlreadyExistsError,
            _ => CreateUserError::DatabaseError(value),
        }
    }
}

pub async fn create_user(
    data: CreateUserData<'_>,
    db: &Database,
) -> Result<AuthUser, CreateUserError> {
    let user = query_as!(
        AuthUser,
        "INSERT INTO users (email, password) VALUES ($1, $2) RETURNING id, password",
        data.email,
        data.password,
    )
    .fetch_one(db)
    .await?;
    Ok(user)
}

#[derive(Debug)]
pub struct GetUserError(SqlxError);

impl Error for GetUserError {}

impl Display for GetUserError {
    fn fmt(&self, f: &mut Formatter<'_>) -> FormatResult {
        write!(f, "Database error: {}", self.0)
    }
}

impl From<SqlxError> for GetUserError {
    fn from(value: SqlxError) -> Self {
        GetUserError(value)
    }
}

pub async fn get_auth_user_by_id(
    id: &i32,
    db: &Database,
) -> Result<Option<AuthUser>, GetUserError> {
    let user = query_as!(AuthUser, "SELECT id, password FROM users WHERE id = $1", id)
        .fetch_optional(db)
        .await?;
    Ok(user)
}

pub async fn get_auth_user_by_email(
    email: &str,
    db: &Database,
) -> Result<Option<AuthUser>, GetUserError> {
    let user = query_as!(
        AuthUser,
        "SELECT id, password FROM users WHERE email = $1",
        email
    )
    .fetch_optional(db)
    .await?;
    Ok(user)
}
