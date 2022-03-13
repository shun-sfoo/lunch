use clap::Parser;
use rust_embed::RustEmbed;
use std::{env, ffi::OsStr, net::SocketAddr, path::Path, sync::Arc};

use axum::{
    body::{boxed, Body, Full},
    extract::Extension,
    handler::Handler,
    http::{header, Method, Request, Response, StatusCode, Uri},
    response::{Html, IntoResponse},
    routing::get,
    Json, Router, Server,
};
use error::AppError;
use serde::{Deserialize, Serialize};
use tera::Tera;
use tokio::fs;
use tower::ServiceExt;
use tower_http::{
    cors::{Any, CorsLayer},
    services::{ServeDir, ServeFile},
};
use tracing::info;
mod error;

#[derive(RustEmbed)]
#[folder = "my-app/build/"]
struct Asset;

#[derive(Parser, Debug)]
struct Args {
    #[clap(short, long, default_value = "9000")]
    port: u16,
    #[clap(short, long, default_value = ".")]
    root_dir: String,
}

struct StaticServerConfig {
    pub(crate) root_dir: String,
}

#[tokio::main]
async fn main() {
    // 设置日志，测试等级为debug
    env::set_var("RUST_LOG", "debug");
    tracing_subscriber::fmt::init();

    let args = Args::parse();
    tracing::debug!(?args);

    let mut root_dir = args.root_dir;
    if root_dir != "/" {
        root_dir = root_dir.trim_end_matches('/').to_string();
    }

    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(vec![Method::GET]);

    let app = Router::new()
        .route("/", get(index_handler))
        .route("/index_or_content", get(index_or_content))
        .route("/file", get(file))
        .layer(cors)
        .fallback(static_handler.into_service())
        .layer(Extension(Arc::new(StaticServerConfig { root_dir })));

    let addr = SocketAddr::from(([0, 0, 0, 0], 9000));

    Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

#[derive(Serialize, Deserialize, Debug)]
struct FileInfo {
    name: String,
    path_uri: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct PathInfo {
    name: String,
    path_uri: String,
    ext: String,
    is_file: bool,
    last_modified: i64,
}

async fn index_handler() -> impl IntoResponse {
    static_handler("/index.html".parse::<Uri>().unwrap()).await
}

async fn static_handler(uri: Uri) -> impl IntoResponse {
    let mut path = uri.path().trim_start_matches('/').to_string();
    if path.starts_with("dist/") {
        path = path.replace("dist/", "");
    }

    StaticFile(path)
}

pub struct StaticFile<T>(pub T);

impl<T> IntoResponse for StaticFile<T>
where
    T: Into<String>,
{
    fn into_response(self) -> axum::response::Response {
        let path = self.0.into();

        match Asset::get(path.as_str()) {
            Some(content) => {
                let body = boxed(Full::from(content.data));
                let mime = mime_guess::from_path(path).first_or_octet_stream();
                Response::builder()
                    .header(header::CONTENT_TYPE, mime.as_ref())
                    .body(body)
                    .unwrap()
            }
            None => Response::builder()
                .status(StatusCode::NOT_FOUND)
                .body(boxed(Full::from("404")))
                .unwrap(),
        }
    }
}

async fn file(
    axum::extract::Query(data): axum::extract::Query<PathInfo>,
    Extension(cfg): Extension<Arc<StaticServerConfig>>,
) -> impl IntoResponse {
    tracing::debug!(?data);
    // let p = ServeDir::new(".").o

    let res = ServeFile::new((&cfg.root_dir).to_string() + "/" + &data.name);
    // let t = res.map(axum::body::boxed);
    // 去看 notion 文档
    todo!()
}

async fn index_or_content(Extension(cfg): Extension<Arc<StaticServerConfig>>) -> impl IntoResponse {
    let path = Path::new(&cfg.root_dir);
    let mut dir = fs::read_dir(path)
        .await
        .map_err(|_| AppError::Path(&cfg.root_dir.into()))
        .unwrap();

    let mut files: Vec<PathInfo> = Vec::new();

    while let Some(child) = dir.next_entry().await.unwrap() {
        let name = child.file_name().to_string_lossy().to_string();
        let path_uri = name;
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
