use anyhow::Result;
use axum::{
    extract::{Path, State},
    response::Html,
    routing::get,
    Router,
};
use std::{net::SocketAddr, path::PathBuf, sync::Arc};
use tokio::net::TcpListener;
use tower_http::services::ServeDir;
use tracing::info;

#[derive(Debug, Clone)]
struct HttpServerState {
    path: PathBuf,
}

enum PathType {
    NotFound,
    IsDir,
    IsFile,
}

pub async fn process_http_serve(path: PathBuf, port: u16) -> Result<()> {
    let addr = SocketAddr::from(([0, 0, 0, 0], port));
    info!("Serving {:?} on port {}", path, addr);
    let state = HttpServerState { path: path.clone() };
    let dir_service = ServeDir::new(path)
        .append_index_html_on_directories(true)
        .precompressed_br()
        .precompressed_deflate()
        .precompressed_gzip()
        .precompressed_zstd();
    let router = Router::new()
        .nest_service("/tower", dir_service)
        .route("/*path", get(file_handler))
        .with_state(Arc::new(state));
    let listener = TcpListener::bind(addr).await?;
    axum::serve(listener, router).await?;
    Ok(())
}

async fn file_handler(
    State(state): State<Arc<HttpServerState>>,
    Path(path): Path<String>,
) -> Html<String> {
    let p = std::path::Path::new(&state.path).join(path);
    info!("{:?}", p);
    if !p.exists() {
        return gen_resp(p, PathType::NotFound).await;
    } else {
        if p.is_dir() {
            return gen_resp(p, PathType::IsDir).await;
        } else {
            return gen_resp(p, PathType::IsFile).await;
        }
    }
}

async fn gen_resp(path: PathBuf, status: PathType) -> Html<String> {
    let ret = match status {
        PathType::NotFound => "Not Found".into(),
        PathType::IsDir => {
            let mut files = Vec::with_capacity(128);
            let dir = std::fs::read_dir(path).unwrap();
            for item in dir {
                let item = item.unwrap().path();
                files.push(item);
            }
            let mut li_list = String::new();
            for file in files {
                let li = format!("<li>{}</li>", file.to_str().unwrap());
                li_list.push_str(li.as_str())
            }
            let ul = format!("<ul>{}</ul>", li_list);
            ul
        }
        PathType::IsFile => match tokio::fs::read_to_string(path).await {
            Ok(ret) => ret,
            Err(e) => {
                let mut err = String::new();
                err.push_str("Serve Error: ");
                err.push_str(&e.to_string());
                err
            }
        },
    };

    let html = format!(
        r#"<!DOCTYPE html>
            <html lang="en">
            <head>
                <meta charset="UTF-8">
                <meta name="viewport" content="width=device-width, initial-scale=1.0">
                <title>Hello World!</title>
            </head>
            <body>
            {}
            </body>
        </html>"#,
        ret
    );
    Html(html)
}
