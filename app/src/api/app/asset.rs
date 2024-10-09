use crate::state::AppState;
use axum::{
    extract::Request,
    http::header::{CACHE_CONTROL, ETAG},
    response::IntoResponse,
    routing::get,
    Router,
};
use once_cell::sync::Lazy;
use regex::Regex;
use tower::ServiceExt;
use tower_http::services::ServeDir;
use tracing::warn;

const ASSET_CACHE_CONTROL_HEADER: &str = "public, max-age=31536000, immutable";

static ASSET_WITH_ETAG_REGEX: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"^[^.]+\.(?P<etag>[^.]+)\.[^.]+$").expect("Wrong regex pattern"));

pub fn create_assets_router() -> Router<AppState> {
    Router::new()
        .nest_service("/styles", get(serve_styles))
        .nest_service("/scripts", get(serve_scripts))
}

async fn serve_styles(request: Request) -> impl IntoResponse {
    serve_dir("dist/styles", request).await
}

async fn serve_scripts(request: Request) -> impl IntoResponse {
    serve_dir("dist/scripts", request).await
}

async fn serve_dir(path: &str, request: Request) -> impl IntoResponse {
    let extracted_etag = extract_etag(request.uri().path()).map(|hash| hash.to_owned());
    let result = ServeDir::new(path).oneshot(request).await;
    result.map(|mut response| {
        if response.status().is_success() {
            if let Some(etag) = extracted_etag {
                response.headers_mut().insert(
                    CACHE_CONTROL,
                    ASSET_CACHE_CONTROL_HEADER
                        .parse()
                        .expect("Invalid header value"),
                );
                response
                    .headers_mut()
                    .insert(ETAG, etag.parse().expect("Invalid header value"));
            } else {
                warn!("Asset {} doesn't contain etag", path)
            }
        }
        response
    })
}

fn extract_etag(asset: &str) -> Option<&str> {
    ASSET_WITH_ETAG_REGEX
        .captures(asset)
        .map(|captures| captures.name("etag").map(|etag| etag.as_str()))?
}
