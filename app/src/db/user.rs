use crate::db::connection::Database;
use sqlx::{query_as, Error as SqlxError};
use std::error::Error;
use std::fmt::{Debug as FormatDebug, Display, Formatter, Result as FormatResult};

#[derive(Clone)]
pub struct User {
    pub id: i32,
    pub password: String,
}

impl FormatDebug for User {
    fn fmt(&self, f: &mut Formatter<'_>) -> FormatResult {
        f.debug_struct("User")
            .field("id", &self.id)
            .field("password", &"********")
            .finish()
    }
}

#[derive(Debug)]
pub enum CreateUserError {
    EmailAlreadyExistsError,
    DatabaseError(SqlxError),
}

impl Error for CreateUserError {}

impl From<SqlxError> for CreateUserError {
    fn from(value: SqlxError) -> Self {
        match value.as_database_error() {
            Some(e) if e.is_unique_violation() => CreateUserError::EmailAlreadyExistsError,
            _ => CreateUserError::DatabaseError(value),
        }
    }
}

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

pub struct CreateUserData<'a> {
    pub email: &'a str,
    pub password: &'a str,
}

pub async fn create_user(data: CreateUserData<'_>, db: &Database) -> Result<User, CreateUserError> {
    let user = query_as!(
        User,
        "INSERT INTO users (email, password) VALUES ($1, $2) RETURNING id, password",
        data.email,
        data.password,
    )
    .fetch_one(db)
    .await?;
    Ok(user)
}

#[derive(Debug)]
pub enum GetUserError {
    DatabaseError(SqlxError),
}

impl Error for GetUserError {}

impl From<SqlxError> for GetUserError {
    fn from(value: SqlxError) -> Self {
        GetUserError::DatabaseError(value)
    }
}

impl Display for GetUserError {
    fn fmt(&self, f: &mut Formatter<'_>) -> FormatResult {
        match self {
            GetUserError::DatabaseError(e) => {
                write!(f, "Database error: {}", e)
            }
        }
    }
}

pub async fn get_user_by_id(id: &i32, db: &Database) -> Result<Option<User>, GetUserError> {
    let user = query_as!(User, "SELECT id, password FROM users WHERE id = $1", id)
        .fetch_optional(db)
        .await?;
    Ok(user)
}

pub async fn get_user_by_email(email: &str, db: &Database) -> Result<Option<User>, GetUserError> {
    let user = query_as!(
        User,
        "SELECT id, password FROM users WHERE email = $1",
        email
    )
    .fetch_optional(db)
    .await?;
    Ok(user)
}
