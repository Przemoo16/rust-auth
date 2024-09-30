use app::db::connection::Database;
use axum::{
    body::Body,
    extract::Request,
    http::{
        header::{CACHE_CONTROL, COOKIE, LOCATION},
        StatusCode,
    },
};
use tower::ServiceExt;

mod common;
use common::{create_test_router, get_authenticated_user_cookie, is_html_response};

#[sqlx::test]
async fn get_protected_page(db: Database) {
    let router = create_test_router(db).await;
    let auth_cookie = get_authenticated_user_cookie(router.clone()).await;

    let response = router
        .oneshot(
            Request::builder()
                .uri("/protected")
                .header(COOKIE, auth_cookie)
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);
    assert!(is_html_response(&response));
}

#[sqlx::test]
async fn get_protected_page_redirect_on_not_authenticated(db: Database) {
    let router = create_test_router(db).await;

    let response = router
        .oneshot(
            Request::builder()
                .uri("/protected")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::TEMPORARY_REDIRECT);
    assert_eq!(
        response.headers().get(LOCATION).unwrap(),
        "/signin?next=%2Fprotected"
    );
}

#[sqlx::test]
async fn dont_cache_protected_pages(db: Database) {
    let router = create_test_router(db).await;
    let auth_cookie = get_authenticated_user_cookie(router.clone()).await;

    let response = router
        .oneshot(
            Request::builder()
                .uri("/protected")
                .header(COOKIE, auth_cookie)
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);
    assert_eq!(
        response.headers().get(CACHE_CONTROL).unwrap(),
        "no-cache, private"
    );
}
