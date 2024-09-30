use app::{
    config::Config,
    db::connection::{setup_session_store, Database},
    server::create_router,
};
use axum::{
    body::Body,
    extract::Request,
    http::{
        header::{CONTENT_TYPE, SET_COOKIE},
        HeaderValue, Method, StatusCode,
    },
    response::Response,
    Router,
};
use mime::{APPLICATION_WWW_FORM_URLENCODED, TEXT_HTML_UTF_8};
use tower::ServiceExt;
use urlencoding::encode;

pub async fn create_test_router(db: Database) -> Router {
    // TODO: Improve performance by cloning database (available for Postgres) instead
    // of recreating db with all migrations for each test.
    let config = Config::from_env();
    let session_store = setup_session_store(db.clone()).await;
    create_router(&config, db, session_store)
}

pub fn is_html_response(response: &Response) -> bool {
    response
        .headers()
        .get(CONTENT_TYPE)
        .map_or(false, |content_type| {
            content_type == TEXT_HTML_UTF_8.as_ref()
        })
}

pub async fn get_authenticated_user_cookie(router: Router) -> HeaderValue {
    let form_data = format!(
        "email={}&password={}&confirm_password={}",
        encode("test@example.com"),
        encode("password123"),
        encode("password123")
    );

    let response = router
        .oneshot(
            Request::builder()
                .method(Method::POST)
                .uri("/signup")
                .header(CONTENT_TYPE, APPLICATION_WWW_FORM_URLENCODED.as_ref())
                .body(Body::from(form_data))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::CREATED);
    response.headers().get(SET_COOKIE).unwrap().to_owned()
}
