use axum::{http::StatusCode, response::IntoResponse};

const PAGE_CONTENT_SELECTOR: &str = "#page";

pub fn create_redirect_response(status_code: StatusCode, path: &str) -> impl IntoResponse {
    (
        status_code,
        [(
            "HX-Location",
            format!(
                r#"{{"path":"{}","target":"{}"}}"#,
                path, PAGE_CONTENT_SELECTOR
            ),
        )],
    )
        .into_response()
}
