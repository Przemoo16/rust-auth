use crate::db::connection::Database;
use sqlx::query_as;

pub enum CreateUserError {
    EmailAlreadyExistsError,
    DatabaseError,
}

pub struct CreateUserData<'a> {
    pub email: &'a str,
    pub password: &'a str,
}

pub async fn create_user(data: CreateUserData<'_>, db: &Database) -> Result<(), CreateUserError> {
    query_as!(
        User,
        "INSERT INTO users (email, password) VALUES ($1, $2)",
        data.email,
        data.password
    )
    .execute(db)
    .await
    .map_err(|e| match e.as_database_error() {
        Some(e) if e.is_unique_violation() => CreateUserError::EmailAlreadyExistsError,
        _ => CreateUserError::DatabaseError,
    })?;
    Ok(())
}
