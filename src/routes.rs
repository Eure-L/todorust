use crate::api::{add_task_handle, delete_task_handle, index};
use axum::routing::{delete, get, post};
use axum::Router;

pub fn get_routes() -> Router {
    Router::new()
        .route("/", get(index))
        .route("/tasks/add", post(add_task_handle))
        .route("/tasks/delete", delete(delete_task_handle))
}
