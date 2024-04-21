use axum::Router;
use axum::routing::{delete, get, post};
use crate::{add_task_handle, delete_task_handle, index};

pub fn get_router() -> Router {
    Router::new()
        .route("/", get(index))
        .route("/tasks/add", post(add_task_handle))
        .route("/tasks/delete", delete(delete_task_handle))
}
