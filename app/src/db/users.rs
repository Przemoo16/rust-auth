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
    let res = query_as!(
        User,
        "INSERT INTO users (email, password) VALUES ($1, $2)",
        data.email,
        data.password
    )
    .execute(db)
    .await;
    if let Err(e) = res {
        if let Some(e) = e.as_database_error() {
            if e.is_unique_violation() {
                return Err(CreateUserError::EmailAlreadyExistsError);
            }
        }
        return Err(CreateUserError::DatabaseError);
    }
    Ok(())
}
