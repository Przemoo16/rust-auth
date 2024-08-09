use axum::{extract::Request, middleware::Next, response::Response};

#[derive(Clone)]
pub struct RenderOptions {
    pub use_base_layout: bool,
}

pub async fn set_render_options(mut req: Request, next: Next) -> Response {
    let is_htmx_request = req.headers().contains_key("HX-Request");
    let request_info = RenderOptions {
        use_base_layout: !is_htmx_request,
    };
    req.extensions_mut().insert(request_info);
    next.run(req).await
}
