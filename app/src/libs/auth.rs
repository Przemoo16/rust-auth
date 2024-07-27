use async_trait::async_trait;
use axum_login::{AuthUser, AuthnBackend, UserId};

#[derive(Clone)]
pub struct Backend {}

impl Backend {
    pub fn new() -> Self {
        Self {}
    }
}

pub struct Credentials {
    user_id: i64,
}

#[derive(Clone, Debug)]
pub struct User {
    id: i64,
}

impl AuthUser for User {
    type Id = i64;

    fn id(&self) -> Self::Id {
        self.id
    }

    fn session_auth_hash(&self) -> &[u8] {
        "foo".as_bytes()
    }
}

#[async_trait]
impl AuthnBackend for Backend {
    type User = User;
    type Credentials = Credentials;
    type Error = std::convert::Infallible;

    async fn authenticate(
        &self,
        Credentials { user_id: _ }: Self::Credentials,
    ) -> Result<Option<Self::User>, Self::Error> {
        Ok(Some(User { id: 1 }))
    }

    async fn get_user(&self, _user_id: &UserId<Self>) -> Result<Option<Self::User>, Self::Error> {
        Ok(Some(User { id: 1 }))
    }
}
