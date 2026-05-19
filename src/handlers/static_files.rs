use std::fs;
use std::path::{Path, PathBuf};
use crate::http::{Request, Response, StatusCode};
use crate::http::mime::get_content_type;
use crate::handlers::not_found;

const STATIC_ROOT: &str = "static";

pub fn index(_req: &Request) -> Response {
    serve("static/index.html")
}

pub fn about(_req: &Request) -> Response {
    serve("static/about.html")
}

pub fn assets(req: &Request) -> Response {
    let rel = req.path.trim_start_matches('/');
    serve(rel)
}

pub fn serve(file_path: &str) -> Response {
    let safe = match resolve_safe(file_path) {
        Some(p) => p,
        None => return not_found::serve(),
    };

    match fs::read(&safe) {
        Ok(bytes) => {
            let mime = get_content_type(safe.to_str().unwrap_or(""));
            Response::new(StatusCode::Ok, bytes, mime)
        }
        Err(_) => Response::new(
            StatusCode::InternalServerError,
            "File read error",
            "text/plain",
        ),
    }
}

fn resolve_safe(file_path: &str) -> Option<PathBuf> {
    let root = Path::new(STATIC_ROOT).canonicalize().ok()?;
    let target = Path::new(file_path).canonicalize().ok()?;
    if target.starts_with(&root) && target.is_file() {
        Some(target)
    } else {
        None
    }
}
