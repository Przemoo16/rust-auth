use crate::db::connection::Database;
use sqlx::{query, query_as, Error as SqlxError};
use std::error::Error;
use std::fmt::{Display, Formatter, Result as FormatResult};

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

pub async fn create_user(data: CreateUserData<'_>, db: &Database) -> Result<(), CreateUserError> {
    query!(
        "INSERT INTO users (email, password) VALUES ($1, $2)",
        data.email,
        data.password
    )
    .execute(db)
    .await?;
    Ok(())
}

#[derive(Debug, Clone)]
pub struct User {
    pub id: i32,
    pub password: String,
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
