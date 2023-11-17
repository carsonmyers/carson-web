use axum::body::{boxed, Body};
use axum::http::{Response, StatusCode};
use axum::{routing::get, Router};
use tower::ServiceExt;
use tower_http::services::ServeDir;

#[shuttle_runtime::main]
async fn main() -> shuttle_axum::ShuttleAxum {
    let api_router = Router::new();

    let router = Router::new()
        .nest("/api", api_router)
        .fallback_service(get(|req| async move {
            match ServeDir::new("frontend/dist").oneshot(req).await {
                Ok(res) => res.map(boxed),
                Err(err) => Response::builder()
                    .status(StatusCode::NOT_FOUND)
                    .body(boxed(Body::from(format!("error: {err}"))))
                    .expect("error response"),
            }
        }));

    Ok(router.into())
}
