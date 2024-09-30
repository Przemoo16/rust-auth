use app::{db::connection::Database, libs::asset::get_asset_path};
use axum::{
    body::Body,
    extract::Request,
    http::{
        header::{CACHE_CONTROL, ETAG},
        StatusCode,
    },
};
use tower::ServiceExt;

pub mod common;
use common::create_test_router;

#[sqlx::test]
async fn get_styles(db: Database) {
    let router = create_test_router(db).await;
    let path = get_asset_path("styles/main.css").unwrap();

    let response = router
        .oneshot(
            Request::builder()
                .uri(format!("/{}", path))
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);
    assert_eq!(
        response.headers().get(CACHE_CONTROL).unwrap(),
        "public, max-age=31536000, immutable"
    );
    assert!(response.headers().contains_key(ETAG));
}

#[sqlx::test]
async fn get_scripts(db: Database) {
    let router = create_test_router(db).await;
    let path = get_asset_path("scripts/main.js").unwrap();

    let response = router
        .oneshot(
            Request::builder()
                .uri(format!("/{}", path))
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);
    assert_eq!(
        response.headers().get(CACHE_CONTROL).unwrap(),
        "public, max-age=31536000, immutable"
    );
    assert!(response.headers().contains_key(ETAG));
}
