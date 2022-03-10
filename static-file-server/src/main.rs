use std::{env, ffi::OsStr, net::SocketAddr, path::Path};

use axum::{
    body::Body,
    extract::Extension,
    http::{Method, Request, StatusCode},
    response::{Html, IntoResponse},
    routing::get,
    Json, Router, Server,
};
use error::AppError;
use serde::Serialize;
use tera::Tera;
use tokio::fs;
use tower::ServiceExt;
use tower_http::{
    cors::{Any, CorsLayer},
    services::ServeDir,
};
use tracing::info;
mod error;

#[tokio::main]
async fn main() {
    // 设置日志，测试等级为debug
    env::set_var("RUST_LOG", "debug");
    dotenv::dotenv().ok();

    tracing_subscriber::fmt::init();
    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(vec![Method::GET]);

    let app = Router::new()
        .route("/", get(index_or_content))
        .layer(cors)
        .fallback(get(download));

    let addr = SocketAddr::from(([0, 0, 0, 0], 9000));

    Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

#[derive(Serialize)]
struct FileInfo {
    name: String,
    path_uri: String,
}

#[derive(Serialize)]
struct PathInfo {
    name: String,
    path_uri: String,
    ext: String,
    is_file: bool,
    last_modified: i64,
}

async fn index_or_content() -> impl IntoResponse {
    let local_dir = ".";
    let path = Path::new(&local_dir);
    let mut dir = fs::read_dir(path)
        .await
        .map_err(|_| AppError::Path(local_dir.into()))
        .unwrap();

    let mut files: Vec<PathInfo> = Vec::new();

    while let Some(child) = dir.next_entry().await.unwrap() {
        let name = child.file_name().to_string_lossy().to_string();
        let path_uri = format!("images/{}", name);
        let ext = Path::new(child.file_name().to_str().unwrap())
            .extension()
            .and_then(OsStr::to_str)
            .unwrap_or_default()
            .to_string();
        let is_file = child.file_type().await.unwrap().is_file();
        let last_modified = child
            .metadata()
            .await
            .unwrap()
            .modified()
            .unwrap()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs() as i64;

        files.push(PathInfo {
            name,
            path_uri,
            ext,
            is_file,
            last_modified,
        });
    }

    Json(files)
}

async fn download(req: Request<Body>) -> impl IntoResponse {
    let image_dir = env::var("IMAGE_DIR")
        .map_err(|_| AppError::MissingEnvParam("IMAGE_DIR".into()))
        .unwrap();
    let image_dir = &image_dir[0..image_dir.len() - 7];
    info!(?image_dir);
    let path = req.uri().path().to_string();
    info!(?path);

    return match ServeDir::new(image_dir).oneshot(req).await {
        Ok(res) => Ok(res.map(axum::body::boxed)),
        Err(e) => Err(format!("{}", e)),
    };
}

async fn list(
    Extension(ref template): Extension<Tera>,
) -> Result<Html<String>, (StatusCode, &'static str)> {
    let image_dir = env::var("IMAGE_DIR")
        .map_err(|_| AppError::MissingEnvParam("IMAGE_DIR".into()))
        .unwrap();

    let path = Path::new(&image_dir);

    let mut dir = fs::read_dir(path)
        .await
        .map_err(|_| AppError::Path(image_dir.into()))
        .unwrap();

    let mut files: Vec<FileInfo> = Vec::new();

    while let Some(child) = dir.next_entry().await.unwrap() {
        let name = child.file_name().to_string_lossy().to_string();
        let path_uri = format!("images/{}", name);
        files.push(FileInfo { name, path_uri });
    }

    let mut ctx = tera::Context::new();
    ctx.insert("files", &files);

    let body = template
        .render("list.html", &ctx)
        .map_err(|_| (StatusCode::INTERNAL_SERVER_ERROR, "template Error"))?;

    Ok(Html(body))
}
