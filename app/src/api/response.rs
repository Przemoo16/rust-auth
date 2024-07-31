use axum::{http::StatusCode, response::IntoResponse};

const MAIN_CONTENT_SELECTOR: &str = "#main";

pub fn create_redirect_response(status_code: StatusCode, path: &str) -> impl IntoResponse {
    (
        status_code,
        [(
            "HX-Location",
            format!(
                r#"{{"path":"{}","target":"{}"}}"#,
                path, MAIN_CONTENT_SELECTOR
            ),
        )],
    )
        .into_response()
}
