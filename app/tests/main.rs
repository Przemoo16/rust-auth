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

pub mod common;
use common::{create_test_router, get_authenticated_user_cookie, is_html_response};

#[sqlx::test]
async fn get_home_page(db: Database) {
    let router = create_test_router(db).await;

    let response = router
        .oneshot(Request::builder().uri("/").body(Body::empty()).unwrap())
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);
    assert!(is_html_response(&response));
}

#[sqlx::test]
async fn get_home_page_redirect_on_authenticated(db: Database) {
    let router = create_test_router(db).await;
    let auth_cookie = get_authenticated_user_cookie(router.clone()).await;

    let response = router
        .oneshot(
            Request::builder()
                .uri("/")
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
async fn get_404_page(db: Database) {
    let router = create_test_router(db).await;

    let response = router
        .oneshot(
            Request::builder()
                .uri("/non-existing-page")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::NOT_FOUND);
    assert!(is_html_response(&response));
}

#[sqlx::test]
async fn dont_cache_pages_by_default(db: Database) {
    let router = create_test_router(db).await;

    let response = router
        .oneshot(Request::builder().uri("/").body(Body::empty()).unwrap())
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);
    assert_eq!(response.headers().get(CACHE_CONTROL).unwrap(), "no-cache");
}
