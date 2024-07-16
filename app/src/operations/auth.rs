use crate::db::connection::Database;
use crate::db::users::{create_user, CreateUserData, CreateUserError};
use crate::libs::auth::hash_password;
use tokio::task;

pub enum SignupError {
    HashPasswordError,
    UserAlreadyExistsError,
    DatabaseError,
}

pub struct SignupData<'a> {
    pub email: &'a str,
    pub password: String,
}

pub async fn signup(data: SignupData<'_>, db: &Database) -> Result<(), SignupError> {
    let hashed_password = task::spawn_blocking(move || hash_password(&data.password))
        .await
        .map_err(|_| SignupError::HashPasswordError)?
        .map_err(|_| SignupError::HashPasswordError)?;

    create_user(
        CreateUserData {
            email: data.email,
            password: &hashed_password,
        },
        db,
    )
    .await
    .map_err(|e| match e {
        CreateUserError::EmailAlreadyExistsError => SignupError::UserAlreadyExistsError,
        _ => SignupError::DatabaseError,
    })?;
    // TODO: Login here
    Ok(())
}
