use axum::{
    http::StatusCode,
    response::{IntoResponse, Redirect},
};

use crate::api::constant::PROTECTED_ROUTE;

const PAGE_CONTENT_SELECTOR: &str = "#page";

pub fn create_client_side_redirect(status_code: StatusCode, path: &str) -> impl IntoResponse {
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

pub fn create_redirect_for_authenticated() -> Redirect {
    Redirect::temporary(PROTECTED_ROUTE)
}
