use std::{net::SocketAddr, path::PathBuf, sync::Arc};

use anyhow::Result;
use axum::{
    body::Body,
    extract::{Path, State},
    http::StatusCode,
    response::{Html, IntoResponse, Response},
    routing::get,
    Router,
};
use tower_http::services::ServeDir;
use tracing::{info, warn};

#[derive(Debug)]
struct HttpServeState {
    path: PathBuf,
}

pub async fn process_http_serve(path: PathBuf, port: u16) -> Result<()> {
    let addr = SocketAddr::from(([0, 0, 0, 0], port));
    info!("Serving {:?} on port {:?}", path, addr);

    let state = HttpServeState { path: path.clone() };

    let dir_service = ServeDir::new(path)
        .append_index_html_on_directories(true)
        .precompressed_br()
        .precompressed_deflate()
        .precompressed_gzip()
        .precompressed_zstd();
    // axum router
    let router = Router::new()
        .nest_service("/tower", dir_service)
        .route("/*path", get(file_handler))
        .with_state(Arc::new(state));

    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, router).await?;
    Ok(())
}

async fn file_handler(
    State(state): State<Arc<HttpServeState>>,
    Path(path): Path<String>,
) -> Response<Body> {
    format!("{:?}, {}", state, path);
    let p = std::path::Path::new(&state.path).join(path);
    info!("Reading file {:?}", p);
    if !p.exists() {
        (
            StatusCode::NOT_FOUND,
            [("content-type", "text/plain")],
            format!("File {} not found!", p.display()),
        )
            .into_response()
    } else {
        // TODO: test p is a directory
        if p.is_dir() {
            let mut content = String::new();
            content.push_str(&format!(
                "<!DOCTYPE html>
<html lang=\"en\">
    <head>
        <meta charset=\"UTF-8\">
        <meta name=\"viewport\" content=\"width=device-width, initial-scale=1.0\">
        <title>Directory Listing</title>
    </head>
    <body>
        <h1>Files in {}</h1>
        <ul>\n",
                p.display()
            ));
            // if it is a directory, list all files/subdirectories
            // as <li><a href="/path/to/file">file name</a></li>
            let entries_result = tokio::fs::read_dir(p).await;
            match entries_result {
                Ok(mut entries) => {
                    loop {
                        let entry_result = entries.next_entry().await;
                        match entry_result {
                            Ok(Some(entry)) => {
                                let name = entry.file_name();
                                let name = name.to_string_lossy();
                                let path = entry.path();
                                let path = path.strip_prefix(&state.path).unwrap();
                                let path = path.to_string_lossy();
                                content.push_str(&format!(
                                    "\t\t<li><a href=\"{}\">{}</a></li>\n",
                                    path, name
                                ));
                            }
                            Ok(None) => break,
                            Err(e) => {
                                warn!("Error reading entry: {:?}", e);
                                return (StatusCode::INTERNAL_SERVER_ERROR, e.to_string())
                                    .into_response();
                            }
                        }
                    }
                    content.push_str(
                        "        </ul>
    </body>
</html>",
                    );
                    (StatusCode::OK, Html(content)).into_response()
                }
                Err(e) => {
                    warn!("Error reading directory: {:?}", e);
                    (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response()
                }
            }
        } else {
            match tokio::fs::read_to_string(p).await {
                Ok(content) => {
                    info!("Read {} bytes", content.len());
                    (StatusCode::OK, content).into_response()
                }
                Err(e) => {
                    warn!("Error reading file: {:?}", e);
                    (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response()
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_file_handler() {
        let state = Arc::new(HttpServeState {
            path: PathBuf::from("."),
        });
        let path = Path("Cargo.toml".to_string());
        let response = file_handler(State(state), path).await;
        assert_eq!(response.status(), StatusCode::OK);
        // assert!(response.trim().starts_with("[package]"));
    }
}
