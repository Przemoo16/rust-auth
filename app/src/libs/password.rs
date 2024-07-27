use argon2::{
    password_hash::{
        rand_core::OsRng, Error as Argon2Error, PasswordHash, PasswordHasher, PasswordVerifier,
        SaltString,
    },
    Argon2,
};
use std::error::Error;
use std::fmt::{Display, Formatter, Result as FormatResult};
use tokio::task::{spawn_blocking, JoinError};

#[derive(Debug)]
pub enum HashPasswordError {
    ThreadError(JoinError),
    HashError(Argon2Error),
}

impl Error for HashPasswordError {}

impl From<JoinError> for HashPasswordError {
    fn from(value: JoinError) -> Self {
        HashPasswordError::ThreadError(value)
    }
}

impl From<Argon2Error> for HashPasswordError {
    fn from(value: Argon2Error) -> Self {
        HashPasswordError::HashError(value)
    }
}

impl Display for HashPasswordError {
    fn fmt(&self, f: &mut Formatter<'_>) -> FormatResult {
        match self {
            HashPasswordError::ThreadError(e) => write!(f, "Thread error: {}", e),
            HashPasswordError::HashError(e) => write!(f, "Hash error: {}", e),
        }
    }
}

pub async fn hash_password_in_separate_thread(
    password: String,
) -> Result<String, HashPasswordError> {
    let hashed_password = spawn_blocking(move || hash_password(&password.into_bytes())).await??;
    Ok(hashed_password)
}

fn hash_password(password: &[u8]) -> Result<String, Argon2Error> {
    let salt = SaltString::generate(&mut OsRng);
    let hashed_password = Argon2::default()
        .hash_password(password, &salt)?
        .to_string();
    Ok(hashed_password)
}

#[derive(Debug)]
pub enum VerifyPasswordError {
    ThreadError(JoinError),
    HashError(Argon2Error),
}

impl Error for VerifyPasswordError {}

impl From<JoinError> for VerifyPasswordError {
    fn from(value: JoinError) -> Self {
        VerifyPasswordError::ThreadError(value)
    }
}

impl From<Argon2Error> for VerifyPasswordError {
    fn from(value: Argon2Error) -> Self {
        VerifyPasswordError::HashError(value)
    }
}

impl Display for VerifyPasswordError {
    fn fmt(&self, f: &mut Formatter<'_>) -> FormatResult {
        match self {
            VerifyPasswordError::ThreadError(e) => write!(f, "Thread error: {}", e),
            VerifyPasswordError::HashError(e) => write!(f, "Hash error: {}", e),
        }
    }
}

pub async fn verify_password_in_separate_thread(
    password: String,
    hashed_password: String,
) -> Result<bool, VerifyPasswordError> {
    let is_valid_password =
        spawn_blocking(move || verify_password(&password.into_bytes(), &hashed_password)).await??;
    Ok(is_valid_password)
}

fn verify_password(password: &[u8], hashed_password: &str) -> Result<bool, Argon2Error> {
    let parsed_hash = PasswordHash::new(hashed_password)?;
    let is_valid_password = Argon2::default()
        .verify_password(password, &parsed_hash)
        .is_ok();
    Ok(is_valid_password)
}
