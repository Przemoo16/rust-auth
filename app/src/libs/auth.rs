use crate::{
    db::{
        connection::Database,
        user::{get_auth_user_by_email, get_auth_user_by_id, AuthUser, GetUserError},
    },
    libs::password::{
        hash_password_in_separate_thread, verify_password_in_separate_thread, HashPasswordError,
        VerifyPasswordError,
    },
};
use async_trait::async_trait;
use axum_login::{
    AuthSession as BaseAuthSession, AuthUser as BaseAuthUser, AuthnBackend, Error as BaseError,
    UserId,
};
use std::{
    error::Error,
    fmt::{Display, Formatter, Result as FormatResult},
};
use tracing::debug;

#[derive(Clone)]
pub struct Backend {
    db: Database,
}

impl Backend {
    pub fn new(db: Database) -> Self {
        Self { db }
    }
}

impl BaseAuthUser for AuthUser {
    type Id = i32;

    fn id(&self) -> Self::Id {
        self.id
    }

    fn session_auth_hash(&self) -> &[u8] {
        // Use the password hash as the auth hash, so when user changes their
        // password the auth session becomes invalid.
        self.password.as_bytes()
    }
}

#[derive(Debug)]
pub enum AuthenticationError {
    GetUserError(GetUserError),
    VerifyPasswordError(VerifyPasswordError),
    HashPasswordError(HashPasswordError),
}

impl Error for AuthenticationError {}

impl Display for AuthenticationError {
    fn fmt(&self, f: &mut Formatter<'_>) -> FormatResult {
        match self {
            AuthenticationError::GetUserError(e) => {
                write!(f, "Get user error: {}", e)
            }
            AuthenticationError::VerifyPasswordError(e) => {
                write!(f, "Verify password error: {}", e)
            }
            AuthenticationError::HashPasswordError(e) => {
                write!(f, "Hash password error: {}", e)
            }
        }
    }
}

impl From<GetUserError> for AuthenticationError {
    fn from(value: GetUserError) -> Self {
        AuthenticationError::GetUserError(value)
    }
}

impl From<VerifyPasswordError> for AuthenticationError {
    fn from(value: VerifyPasswordError) -> Self {
        AuthenticationError::VerifyPasswordError(value)
    }
}

impl From<HashPasswordError> for AuthenticationError {
    fn from(value: HashPasswordError) -> Self {
        AuthenticationError::HashPasswordError(value)
    }
}

pub struct Credentials {
    pub email: String,
    pub password: String,
}

#[async_trait]
impl AuthnBackend for Backend {
    type User = AuthUser;
    type Credentials = Credentials;
    type Error = AuthenticationError;

    async fn authenticate(
        &self,
        creds: Self::Credentials,
    ) -> Result<Option<Self::User>, Self::Error> {
        if let Some(user) = get_auth_user_by_email(&creds.email, &self.db).await? {
            if verify_password_in_separate_thread(creds.password, user.password.clone()).await? {
                return Ok(Some(user));
            } else {
                debug!("Invalid password for user with email {}", creds.email);
            }
        } else {
            debug!("User with email {} not found", creds.email);
            // Run the password hasher to mitigate timing attack
            hash_password_in_separate_thread(creds.password).await?;
        }
        Ok(None)
    }

    async fn get_user(&self, user_id: &UserId<Self>) -> Result<Option<Self::User>, Self::Error> {
        let user = get_auth_user_by_id(user_id, &self.db).await?;
        Ok(user)
    }
}

pub async fn is_anonymous(auth_session: AuthSession) -> bool {
    auth_session.user.is_none()
}

pub type AuthSession = BaseAuthSession<Backend>;
pub type AuthError = BaseError<Backend>;
