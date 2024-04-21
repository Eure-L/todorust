use crate::{IndexTemplate, NewTask, Task, TaskID, TasksTemplate};
use askama::Template;
use axum::response::{Html, IntoResponse};
use axum::Form;
use lazy_static::lazy_static;
use log::{info, trace, warn};
use std::collections::HashMap;
use std::sync::Mutex;
use axum::http::StatusCode;
use tokio::net::unix::uid_t;
use uuid::Uuid;

lazy_static! {
    static ref TASKS: Mutex<HashMap<uid_t, Task>> = Mutex::new(HashMap::new());
}

pub async fn index() -> Html<String> {
    trace!("index");

    let tasks = get_pending_tasks(&TASKS);
    let template = IndexTemplate { tasks: &tasks };
    trace!("index");
    Html(template.render().unwrap())

}

pub async fn add_task_handle(Form(input): Form<NewTask>) -> Html<String> {
    trace!("add_task {:?}", input);

    // Builds the new task with User's provided info
    // completing it with server assigned uuid
    let new_id = Uuid::new_v4().as_u128() as uid_t;
    let new_task = Task {
        name: input.name,
        id: new_id,
        description: input.description,
    };

    // Adds task to the server DB
    let _ = match TASKS.lock().unwrap().insert(new_id, new_task.clone()) {
        None => {
            info!("Task added {:?}", new_task);
        }
        Some(task) => {
            warn!("Task with id:{new_id} exists already : \n{:?}", task);
            return Html("".parse().unwrap());
        }
    };

    let tasks = get_pending_tasks(&TASKS);
    let template = TasksTemplate { tasks: &tasks };
    Html(template.render().unwrap())
}

pub async fn delete_task_handle(Form(task): Form<TaskID>) ->  impl IntoResponse  {
    trace!("delete_task {:?}", task);

    match TASKS.lock().unwrap().remove(&task.id) {
        //(&task.id) {
        None => {
            warn!("No task found: {:?}", task.id);
            (StatusCode::NOT_FOUND, "Task not found").into_response()
        }
        Some(a) => {
            info!("Task deleted: {:?}", a);
            (StatusCode::OK).into_response()
        }
    }
}

pub fn get_pending_tasks(tasks_mutex: &Mutex<HashMap<uid_t, Task>>) -> Vec<Task> {
    let tasks = tasks_mutex
        .lock()
        .unwrap()
        .iter()
        .map(|(_, task)| task.clone())
        .collect();
    tasks
}
