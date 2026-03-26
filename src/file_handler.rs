use axum::{
    body::Body,
    extract::{Path, Request},
    http::{header, StatusCode},
    response::{IntoResponse, Response},
};
use rust_embed::RustEmbed;

#[cfg(debug_assertions)]
#[derive(RustEmbed)]
#[folder = "target/site/pkg"]
#[include = "*.js"]
#[include = "*.css"]
#[include = "*.wasm"]
pub struct PkgAssets;

#[cfg(not(debug_assertions))]
#[derive(RustEmbed)]
#[folder = "target/site/pkg"]
#[include = "*.js.br"]
#[include = "*.css.br"]
#[include = "*.wasm.br"]
pub struct PkgAssets;

#[derive(RustEmbed)]
#[folder = "public"]
#[include = "*.svg"]
#[include = "*.ico"]
#[include = "imgs/*.png"]
pub struct PublicAssets;

fn accepts_brotli(req: &Request) -> bool {
    req.headers()
        .get(header::ACCEPT_ENCODING)
        .and_then(|h| h.to_str().ok())
        .map(|s| s.contains("br"))
        .unwrap_or(false)
}

pub async fn serve_pkg_file(Path(path): Path<String>, req: Request) -> impl IntoResponse {
    let file_path = path.trim_start_matches('/');

    #[cfg(debug_assertions)]
    {
        // no op just to get rid of the lint
        let _ = req;
        let content = PkgAssets::get(file_path).or_else(|| {
            if file_path.ends_with("_bg.wasm") {
                PkgAssets::get(&file_path.replace("_bg.wasm", ".wasm"))
            } else {
                None
            }
        });

        match content {
            Some(content) => {
                let mime = mime_guess::from_path(file_path).first_or_octet_stream();
                Response::builder()
                    .status(StatusCode::OK)
                    .header(header::CONTENT_TYPE, mime.as_ref())
                    .body(Body::from(content.data.to_vec()))
                    .map(|r| r.into_response())
                    .unwrap_or_else(|e| {
                        tracing::error!("failed to build response: {e}");
                        (StatusCode::INTERNAL_SERVER_ERROR, "server error").into_response()
                    })
            }
            None => (StatusCode::NOT_FOUND, "not found").into_response(),
        }
    }

    #[cfg(not(debug_assertions))]
    {
        let accept_br = accepts_brotli(&req);

        let (content, is_brotli) = if accept_br {
            let br_path = format!("{file_path}.br");
            let content = PkgAssets::get(&br_path).or_else(|| {
                if file_path.ends_with("_bg.wasm") {
                    PkgAssets::get(&file_path.replace("_bg.wasm", ".wasm.br"))
                } else {
                    None
                }
            });
            (content, true)
        } else {
            (None, false)
        };

        match content {
            Some(content) => {
                let original_name = file_path.trim_end_matches(".br");
                let mime = mime_guess::from_path(original_name).first_or_octet_stream();

                let mut builder = Response::builder()
                    .status(StatusCode::OK)
                    .header(header::CONTENT_TYPE, mime.as_ref())
                    .header(header::CACHE_CONTROL, "public, max-age=31536000, immutable");

                if is_brotli {
                    builder = builder
                        .header(header::CONTENT_ENCODING, "br")
                        .header(header::VARY, "Accept-Encoding");
                }

                builder
                    .body(Body::from(content.data.to_vec()))
                    .map(|r| r.into_response())
                    .unwrap_or_else(|e| {
                        tracing::error!("failed to build response: {e}");
                        (StatusCode::INTERNAL_SERVER_ERROR, "server error").into_response()
                    })
            }
            None => (StatusCode::NOT_FOUND, "not found").into_response(),
        }
    }
}

pub async fn serve_favicon_svg(req: Request) -> impl IntoResponse {
    serve_public_file(Path("favicon.svg".to_string()), req).await
}

pub async fn serve_favicon_ico(req: Request) -> impl IntoResponse {
    serve_public_file(Path("favicon.ico".to_string()), req).await
}

pub async fn serve_img_file(Path(path): Path<String>, req: Request) -> impl IntoResponse {
    serve_public_file(Path(format!("imgs/{path}")), req).await
}

pub async fn serve_public_file(Path(path): Path<String>, req: Request) -> impl IntoResponse {
    let file_path = path.trim_start_matches('/');

    match PublicAssets::get(file_path) {
        Some(content) => {
            let mime = mime_guess::from_path(file_path).first_or_octet_stream();
            let accept_br = accepts_brotli(&req);

            let mut builder = Response::builder()
                .status(StatusCode::OK)
                .header(header::CONTENT_TYPE, mime.as_ref())
                .header(header::CACHE_CONTROL, "public, max-age=31536000, immutable");

            if accept_br {
                builder = builder.header(header::VARY, "Accept-Encoding");
            }

            builder
                .body(Body::from(content.data.to_vec()))
                .map(|r| r.into_response())
                .unwrap_or_else(|e| {
                    tracing::error!("failed to build response: {e}");
                    (StatusCode::INTERNAL_SERVER_ERROR, "server error").into_response()
                })
        }
        None => (StatusCode::NOT_FOUND, "not found").into_response(),
    }
}
