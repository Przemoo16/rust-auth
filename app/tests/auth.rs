use app::db::connection::Database;
use axum::{
    body::Body,
    extract::Request,
    http::{
        header::{CONTENT_TYPE, COOKIE, LOCATION, SET_COOKIE},
        Method, StatusCode,
    },
};
use mime::APPLICATION_WWW_FORM_URLENCODED;
use tower::ServiceExt;
use urlencoding::encode;

pub mod common;
use common::{create_test_router, get_authenticated_user_cookie, is_html_response};

struct SignupPayload<'a> {
    email: &'a str,
    password: &'a str,
    confirm_password: &'a str,
}

impl<'a> Default for SignupPayload<'a> {
    fn default() -> Self {
        Self {
            email: "test@example.com",
            password: "password123",
            confirm_password: "password123",
        }
    }
}

impl<'a> SignupPayload<'a> {
    fn to_form_data(&self) -> String {
        format!(
            "email={}&password={}&confirm_password={}",
            encode(self.email),
            encode(self.password),
            encode(self.confirm_password)
        )
    }
}

struct SigninPayload<'a> {
    email: &'a str,
    password: &'a str,
}

impl<'a> Default for SigninPayload<'a> {
    fn default() -> Self {
        Self {
            email: "test@example.com",
            password: "password123",
        }
    }
}

impl<'a> SigninPayload<'a> {
    fn to_form_data(&self) -> String {
        format!(
            "email={}&password={}",
            encode(self.email),
            encode(self.password),
        )
    }
}

#[sqlx::test]
async fn get_signup_page(db: Database) {
    let router = create_test_router(db).await;

    let response = router
        .oneshot(
            Request::builder()
                .uri("/signup")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);
    assert!(is_html_response(&response));
}

#[sqlx::test]
async fn get_signup_page_redirect_on_authenticated(db: Database) {
    let router = create_test_router(db).await;
    let auth_cookie = get_authenticated_user_cookie(router.clone()).await;

    let response = router
        .oneshot(
            Request::builder()
                .uri("/signup")
                .header(COOKIE, auth_cookie)
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::TEMPORARY_REDIRECT);
    assert_eq!(response.headers().get(LOCATION).unwrap(), "/protected");
}

#[sqlx::test]
async fn sign_up(db: Database) {
    let router = create_test_router(db).await;
    let payload = SignupPayload::default();

    let signup_response = router
        .clone()
        .oneshot(
            Request::builder()
                .method(Method::POST)
                .uri("/signup")
                .header(CONTENT_TYPE, APPLICATION_WWW_FORM_URLENCODED.as_ref())
                .body(Body::from(payload.to_form_data()))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(signup_response.status(), StatusCode::CREATED);

    let protected_response = router
        .oneshot(
            Request::builder()
                .uri("/protected")
                .header(COOKIE, signup_response.headers().get(SET_COOKIE).unwrap())
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(protected_response.status(), StatusCode::OK);
}

#[sqlx::test]
async fn sign_up_with_invalid_email_payload(db: Database) {
    let router = create_test_router(db).await;
    let cases = [
        "",
        &format!("{}@email.com", "a".repeat(245)),
        "invalid-email",
    ];

    for case in cases {
        let payload = SignupPayload {
            email: case,
            ..Default::default()
        };

        let response = router
            .clone()
            .oneshot(
                Request::builder()
                    .method(Method::POST)
                    .uri("/signup")
                    .header(CONTENT_TYPE, APPLICATION_WWW_FORM_URLENCODED.as_ref())
                    .body(Body::from(payload.to_form_data()))
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::UNPROCESSABLE_ENTITY);
    }
}

#[sqlx::test]
async fn sign_up_with_invalid_password_payload(db: Database) {
    let router = create_test_router(db).await;
    let cases = ["", "a", &"a".repeat(257)];

    for case in cases {
        let payload = SignupPayload {
            password: case,
            confirm_password: case,
            ..Default::default()
        };

        let response = router
            .clone()
            .oneshot(
                Request::builder()
                    .method(Method::POST)
                    .uri("/signup")
                    .header(CONTENT_TYPE, APPLICATION_WWW_FORM_URLENCODED.as_ref())
                    .body(Body::from(payload.to_form_data()))
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::UNPROCESSABLE_ENTITY);
    }
}

#[sqlx::test]
async fn sign_up_with_invalid_confirm_password_payload(db: Database) {
    let router = create_test_router(db).await;
    let cases = ["", "mismatched-confirm-password"];

    for case in cases {
        let payload = SignupPayload {
            password: "password123",
            confirm_password: case,
            ..Default::default()
        };

        let response = router
            .clone()
            .oneshot(
                Request::builder()
                    .method(Method::POST)
                    .uri("/signup")
                    .header(CONTENT_TYPE, APPLICATION_WWW_FORM_URLENCODED.as_ref())
                    .body(Body::from(payload.to_form_data()))
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::UNPROCESSABLE_ENTITY);
    }
}

#[sqlx::test]
async fn sign_up_with_already_existing_email(db: Database) {
    let router = create_test_router(db).await;
    let payload = SignupPayload::default();

    let response = router
        .clone()
        .oneshot(
            Request::builder()
                .method(Method::POST)
                .uri("/signup")
                .header(CONTENT_TYPE, APPLICATION_WWW_FORM_URLENCODED.as_ref())
                .body(Body::from(payload.to_form_data()))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::CREATED);

    let response = router
        .oneshot(
            Request::builder()
                .method(Method::POST)
                .uri("/signup")
                .header(CONTENT_TYPE, APPLICATION_WWW_FORM_URLENCODED.as_ref())
                .body(Body::from(payload.to_form_data()))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::CONFLICT);
}

#[sqlx::test]
async fn get_signin_page(db: Database) {
    let router = create_test_router(db).await;

    let response = router
        .oneshot(
            Request::builder()
                .uri("/signin")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);
    assert!(is_html_response(&response));
}

#[sqlx::test]
async fn get_signin_page_redirect_on_authenticated(db: Database) {
    let router = create_test_router(db).await;
    let auth_cookie = get_authenticated_user_cookie(router.clone()).await;

    let response = router
        .oneshot(
            Request::builder()
                .uri("/signin")
                .header(COOKIE, auth_cookie)
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::TEMPORARY_REDIRECT);
    assert_eq!(response.headers().get(LOCATION).unwrap(), "/protected");
}

#[sqlx::test]
async fn sign_in(db: Database) {
    let router = create_test_router(db).await;
    let signup_payload = SignupPayload {
        email: "test@example.com",
        password: "password123",
        confirm_password: "password123",
    };

    let signup_response = router
        .clone()
        .oneshot(
            Request::builder()
                .method(Method::POST)
                .uri("/signup")
                .header(CONTENT_TYPE, APPLICATION_WWW_FORM_URLENCODED.as_ref())
                .body(Body::from(signup_payload.to_form_data()))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(signup_response.status(), StatusCode::CREATED);

    let signin_payload = SigninPayload {
        email: "test@example.com",
        password: "password123",
    };

    let signin_response = router
        .clone()
        .oneshot(
            Request::builder()
                .method(Method::POST)
                .uri("/signin")
                .header(CONTENT_TYPE, APPLICATION_WWW_FORM_URLENCODED.as_ref())
                .body(Body::from(signin_payload.to_form_data()))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(signin_response.status(), StatusCode::OK);

    let protected_response = router
        .oneshot(
            Request::builder()
                .uri("/protected")
                .header(COOKIE, signin_response.headers().get(SET_COOKIE).unwrap())
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(protected_response.status(), StatusCode::OK);
}

#[sqlx::test]
async fn sign_in_with_invalid_email_payload(db: Database) {
    let router = create_test_router(db).await;
    let cases = ["", "invalid-email"];

    for case in cases {
        let payload = SigninPayload {
            email: case,
            ..Default::default()
        };

        let response = router
            .clone()
            .oneshot(
                Request::builder()
                    .method(Method::POST)
                    .uri("/signin")
                    .header(CONTENT_TYPE, APPLICATION_WWW_FORM_URLENCODED.as_ref())
                    .body(Body::from(payload.to_form_data()))
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::UNPROCESSABLE_ENTITY);
    }
}

#[sqlx::test]
async fn sign_in_with_invalid_password_payload(db: Database) {
    let router = create_test_router(db).await;
    let payload = SigninPayload {
        password: "",
        ..Default::default()
    };

    let response = router
        .oneshot(
            Request::builder()
                .method(Method::POST)
                .uri("/signin")
                .header(CONTENT_TYPE, APPLICATION_WWW_FORM_URLENCODED.as_ref())
                .body(Body::from(payload.to_form_data()))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::UNPROCESSABLE_ENTITY);
}

#[sqlx::test]
async fn sign_in_with_non_existing_email(db: Database) {
    let router = create_test_router(db).await;
    let payload = SigninPayload::default();

    let response = router
        .oneshot(
            Request::builder()
                .method(Method::POST)
                .uri("/signin")
                .header(CONTENT_TYPE, APPLICATION_WWW_FORM_URLENCODED.as_ref())
                .body(Body::from(payload.to_form_data()))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
}

#[sqlx::test]
async fn sign_in_with_invalid_password(db: Database) {
    let router = create_test_router(db).await;
    let signup_payload = SignupPayload {
        email: "test@example.com",
        password: "password123",
        confirm_password: "password123",
    };

    let signup_response = router
        .clone()
        .oneshot(
            Request::builder()
                .method(Method::POST)
                .uri("/signup")
                .header(CONTENT_TYPE, APPLICATION_WWW_FORM_URLENCODED.as_ref())
                .body(Body::from(signup_payload.to_form_data()))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(signup_response.status(), StatusCode::CREATED);

    let signin_payload = SigninPayload {
        email: "test@example.com",
        password: "invalid-password",
    };

    let signin_response = router
        .oneshot(
            Request::builder()
                .method(Method::POST)
                .uri("/signin")
                .header(CONTENT_TYPE, APPLICATION_WWW_FORM_URLENCODED.as_ref())
                .body(Body::from(signin_payload.to_form_data()))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(signin_response.status(), StatusCode::UNAUTHORIZED);
}

#[sqlx::test]
async fn sign_out(db: Database) {
    let router = create_test_router(db).await;
    let auth_cookie = get_authenticated_user_cookie(router.clone()).await;

    let signout_response = router
        .clone()
        .oneshot(
            Request::builder()
                .method(Method::POST)
                .uri("/signout")
                .header(COOKIE, &auth_cookie)
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(signout_response.status(), StatusCode::NO_CONTENT);

    let protected_response = router
        .oneshot(
            Request::builder()
                .uri("/protected")
                .header(COOKIE, auth_cookie)
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(protected_response.status(), StatusCode::TEMPORARY_REDIRECT);
    assert_eq!(
        protected_response.headers().get(LOCATION).unwrap(),
        "/signin?next=%2Fprotected"
    );
}
