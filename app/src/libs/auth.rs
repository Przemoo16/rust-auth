use argon2::{
    password_hash::{rand_core::OsRng, Error as Argon2Error, PasswordHasher, SaltString},
    Argon2,
};
use std::error::Error;
use std::fmt::{Display, Formatter, Result as FormatResult};
use tokio::task::{spawn_blocking, JoinError};

#[derive(Debug)]
pub enum HashPasswordError {
    HashError(Argon2Error),
    ThreadError(JoinError),
}

impl Error for HashPasswordError {}

impl From<Argon2Error> for HashPasswordError {
    fn from(value: Argon2Error) -> Self {
        HashPasswordError::HashError(value)
    }
}

impl From<JoinError> for HashPasswordError {
    fn from(value: JoinError) -> Self {
        HashPasswordError::ThreadError(value)
    }
}

impl Display for HashPasswordError {
    fn fmt(&self, f: &mut Formatter<'_>) -> FormatResult {
        match self {
            HashPasswordError::HashError(e) => write!(f, "Hash error: {}", e),
            HashPasswordError::ThreadError(e) => write!(f, "Thread error: {}", e),
        }
    }
}

pub async fn hash_password_in_separate_thread(
    password: String,
) -> Result<String, HashPasswordError> {
    let hashed_password = spawn_blocking(move || hash_password(&password)).await??;
    Ok(hashed_password)
}

fn hash_password(password: &str) -> Result<String, Argon2Error> {
    let salt = SaltString::generate(&mut OsRng);
    let hashed_password = Argon2::default()
        .hash_password(password.as_bytes(), &salt)?
        .to_string();
    Ok(hashed_password)
}
