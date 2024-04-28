use std::path::PathBuf;

use axum::Router;
use axum::routing::{delete, get, post, put};
use tower_http::services::ServeDir;

use crate::api::{add_task_handle, delete_task_handle, get_task_info, index, set_task_status_handle};

pub fn get_routes() -> Router {

    let static_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("static");

    Router::new()
        .nest_service(
            "/static",
            ServeDir::new(static_dir),
        )
        .route("/", get(index))
        .route("/tasks/add", post(add_task_handle))
        .route("/tasks/delete", delete(delete_task_handle))
        .route("/tasks/status", put(set_task_status_handle))
        .route("/task/info", get(get_task_info))
}
