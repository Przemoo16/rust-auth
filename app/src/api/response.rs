use axum::{
    http::{
        header::{CACHE_CONTROL, VARY},
        StatusCode,
    },
    response::{IntoResponse, Redirect, Response},
};

use crate::api::constant::HOME_ROUTE;

const PAGE_CONTENT_SELECTOR: &str = "#page";
const PAGE_CACHE_MAX_AGE: u32 = 1 * 24 * 60 * 60;

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
    Redirect::temporary(HOME_ROUTE)
}

pub trait WithCache {
    fn with_cache(self) -> Self;
}

impl WithCache for Response {
    fn with_cache(mut self) -> Self {
        let headers = self.headers_mut();
        headers.insert(
            CACHE_CONTROL,
            format!("max-age={}", PAGE_CACHE_MAX_AGE)
                .parse()
                .expect("Invalid header value"),
        );
        headers.insert(VARY, "HX-Request".parse().expect("Invalid header value"));
        self
    }
}
