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
use std::collections::HashMap;
use tower::ServiceExt;

mod common;
use common::{
    create_form_data, create_test_router, get_authenticated_user_cookie, is_html_response,
};

fn create_signup_data() -> HashMap<&'static str, &'static str> {
    let mut data = HashMap::new();
    data.insert("email", "test@example.pl");
    data.insert("password", "password123");
    data.insert("confirm_password", "password123");
    data
}

fn create_signin_data() -> HashMap<&'static str, &'static str> {
    let mut data = HashMap::new();
    data.insert("email", "test@example.pl");
    data.insert("password", "password123");
    data
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
    let mut data = HashMap::new();
    data.insert("email", "test@example.pl");
    data.insert("password", "password123");
    data.insert("confirm_password", "password123");

    let signup_response = router
        .clone()
        .oneshot(
            Request::builder()
                .method(Method::POST)
                .uri("/signup")
                .header(CONTENT_TYPE, APPLICATION_WWW_FORM_URLENCODED.as_ref())
                .body(Body::from(create_form_data(&data)))
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
async fn sign_up_with_invalid_email_request(db: Database) {
    let router = create_test_router(db).await;
    let cases = [
        "",
        &format!("{}@email.com", "a".repeat(245)),
        "invalid-email",
    ];

    for case in cases {
        let mut data = create_signup_data();
        data.insert("email", case);

        let response = router
            .clone()
            .oneshot(
                Request::builder()
                    .method(Method::POST)
                    .uri("/signup")
                    .header(CONTENT_TYPE, APPLICATION_WWW_FORM_URLENCODED.as_ref())
                    .body(Body::from(create_form_data(&data)))
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::UNPROCESSABLE_ENTITY);
    }
}

#[sqlx::test]
async fn sign_up_with_invalid_password_request(db: Database) {
    let router = create_test_router(db).await;
    let cases = ["", "a", &"a".repeat(257)];

    for case in cases {
        let mut data = create_signup_data();
        data.insert("password", case);
        data.insert("confirm_password", case);

        let response = router
            .clone()
            .oneshot(
                Request::builder()
                    .method(Method::POST)
                    .uri("/signup")
                    .header(CONTENT_TYPE, APPLICATION_WWW_FORM_URLENCODED.as_ref())
                    .body(Body::from(create_form_data(&data)))
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::UNPROCESSABLE_ENTITY);
    }
}

#[sqlx::test]
async fn sign_up_with_invalid_confirm_password_request(db: Database) {
    let router = create_test_router(db).await;
    let cases = ["", "mismatched-confirm-password"];

    for case in cases {
        let mut data = create_signup_data();
        data.insert("password", "password123");
        data.insert("confirm_password", case);

        let response = router
            .clone()
            .oneshot(
                Request::builder()
                    .method(Method::POST)
                    .uri("/signup")
                    .header(CONTENT_TYPE, APPLICATION_WWW_FORM_URLENCODED.as_ref())
                    .body(Body::from(create_form_data(&data)))
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
    let data = create_signup_data();

    let response = router
        .clone()
        .oneshot(
            Request::builder()
                .method(Method::POST)
                .uri("/signup")
                .header(CONTENT_TYPE, APPLICATION_WWW_FORM_URLENCODED.as_ref())
                .body(Body::from(create_form_data(&data)))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::CREATED);

    let response = router
        .clone()
        .oneshot(
            Request::builder()
                .method(Method::POST)
                .uri("/signup")
                .header(CONTENT_TYPE, APPLICATION_WWW_FORM_URLENCODED.as_ref())
                .body(Body::from(create_form_data(&data)))
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
    let mut data = HashMap::new();
    data.insert("email", "test@example.pl");
    data.insert("password", "password123");
    data.insert("confirm_password", "password123");

    let signup_response = router
        .clone()
        .oneshot(
            Request::builder()
                .method(Method::POST)
                .uri("/signup")
                .header(CONTENT_TYPE, APPLICATION_WWW_FORM_URLENCODED.as_ref())
                .body(Body::from(create_form_data(&data)))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(signup_response.status(), StatusCode::CREATED);

    data.remove("confirm_password");
    let signin_response = router
        .clone()
        .oneshot(
            Request::builder()
                .method(Method::POST)
                .uri("/signin")
                .header(CONTENT_TYPE, APPLICATION_WWW_FORM_URLENCODED.as_ref())
                .body(Body::from(create_form_data(&data)))
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
async fn sign_in_with_invalid_email_request(db: Database) {
    let router = create_test_router(db).await;
    let cases = ["", "invalid-email"];

    for case in cases {
        let mut data = create_signin_data();
        data.insert("email", case);

        let response = router
            .clone()
            .oneshot(
                Request::builder()
                    .method(Method::POST)
                    .uri("/signin")
                    .header(CONTENT_TYPE, APPLICATION_WWW_FORM_URLENCODED.as_ref())
                    .body(Body::from(create_form_data(&data)))
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::UNPROCESSABLE_ENTITY);
    }
}

#[sqlx::test]
async fn sign_in_with_invalid_password_request(db: Database) {
    let router = create_test_router(db).await;
    let mut data = create_signin_data();
    data.insert("password", "");

    let response = router
        .oneshot(
            Request::builder()
                .method(Method::POST)
                .uri("/signin")
                .header(CONTENT_TYPE, APPLICATION_WWW_FORM_URLENCODED.as_ref())
                .body(Body::from(create_form_data(&data)))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::UNPROCESSABLE_ENTITY);
}

#[sqlx::test]
async fn sign_in_with_non_existing_email(db: Database) {
    let router = create_test_router(db).await;
    let data = create_signin_data();

    let response = router
        .oneshot(
            Request::builder()
                .method(Method::POST)
                .uri("/signin")
                .header(CONTENT_TYPE, APPLICATION_WWW_FORM_URLENCODED.as_ref())
                .body(Body::from(create_form_data(&data)))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
}

#[sqlx::test]
async fn sign_in_with_invalid_password(db: Database) {
    let router = create_test_router(db).await;
    let mut data = create_signup_data();

    let signup_response = router
        .clone()
        .oneshot(
            Request::builder()
                .method(Method::POST)
                .uri("/signup")
                .header(CONTENT_TYPE, APPLICATION_WWW_FORM_URLENCODED.as_ref())
                .body(Body::from(create_form_data(&data)))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(signup_response.status(), StatusCode::CREATED);

    data.insert("password", "invalid-password");
    data.remove("confirm_password");
    let signin_response = router
        .oneshot(
            Request::builder()
                .method(Method::POST)
                .uri("/signin")
                .header(CONTENT_TYPE, APPLICATION_WWW_FORM_URLENCODED.as_ref())
                .body(Body::from(create_form_data(&data)))
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
