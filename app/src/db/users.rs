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
