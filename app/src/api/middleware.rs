use axum::{extract::Request, http::header::CACHE_CONTROL, response::Response};

#[derive(Clone)]
pub struct RenderOptions {
    pub use_base_layout: bool,
}

pub async fn set_request_render_options<B>(mut request: Request<B>) -> Request<B> {
    let is_htmx_request = request.headers().contains_key("HX-Request");
    let request_info = RenderOptions {
        use_base_layout: !is_htmx_request,
    };
    request.extensions_mut().insert(request_info);
    request
}

pub async fn set_default_response_headers<B>(mut response: Response<B>) -> Response<B> {
    if response.status().is_success() && !response.headers().contains_key(CACHE_CONTROL.as_str()) {
        response.headers_mut().insert(
            CACHE_CONTROL,
            "no-cache".parse().expect("Invalid header value"),
        );
    }
    response
}

pub async fn set_default_response_headers_for_protected<B>(
    mut response: Response<B>,
) -> Response<B> {
    if response.status().is_success() && !response.headers().contains_key(CACHE_CONTROL.as_str()) {
        response.headers_mut().insert(
            CACHE_CONTROL,
            "no-cache, private".parse().expect("Invalid header value"),
        );
    }
    response
}
