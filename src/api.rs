use crate::task::TaskStatus;
use crate::{IndexTemplate, NewTaskForm, Task, TaskDescriptionForm, TaskIdForm, TaskModalTemplate, TaskStatusForm, TaskTemplate};
use askama::Template;
use axum::http::StatusCode;
use axum::response::{Html, IntoResponse};
use axum::Form;
use lazy_static::lazy_static;
use log::{debug, info, trace, warn};
use std::collections::HashMap;
use std::sync::Mutex;
use tokio::net::unix::uid_t;
use uuid::Uuid;

lazy_static! {
    static ref TASKS: Mutex<HashMap<uid_t, Task>> = Mutex::new(HashMap::new());
}

pub async fn index() -> impl IntoResponse {
    trace!("index");

    let tasks = get_tasks(&TASKS);
    let template = IndexTemplate { tasks: &tasks };
    Html(template.render().unwrap()).into_response()
}

pub async fn add_task_handle(Form(input): Form<NewTaskForm>) -> impl IntoResponse {
    trace!("add_task {:?}", input);


    if input.name =="".to_string(){
        warn!("Task submited with no name");
        return (StatusCode::FORBIDDEN, "Task must have a name").into_response();
    }

    // Builds the new task with User's provided info
    // completing it with server assigned uuid
    let new_id = Uuid::new_v4().as_u128() as uid_t;
    let new_task = Task {
        name: input.name,
        id: new_id,
        description: input.description,
        status: TaskStatus::Pending,
    };

    // Adds task to the server DB
    match TASKS.lock().unwrap().insert(new_id, new_task.clone()) {
        None => {
            info!("Task added {:?}", new_task);
            let template = TaskTemplate { task: &new_task };
            Html(template.render().unwrap()).into_response()
        }
        Some(task) => {
            warn!("Task with id:{new_id} exists already : \n{:?}", task);
            (StatusCode::CONFLICT, "Task exists already").into_response()
        }
    }
}

pub async fn delete_task_handle(Form(task): Form<TaskIdForm>) -> impl IntoResponse {
    trace!("delete_task {:?}", task);

    match TASKS.lock().unwrap().remove(&task.id) {
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

pub async fn set_task_status_handle(Form(task): Form<TaskStatusForm>) -> impl IntoResponse {
    trace!("set_task_status {:?}", task);

    for (_, task) in TASKS.lock().unwrap().iter() {
        debug!("{:?}", task)
    }

    match TASKS.lock().unwrap().get_mut(&task.id) {
        None => {
            warn!("No task found: {:?}", task.id);
            (StatusCode::NOT_FOUND, "Task not found").into_response()
        }
        Some(a) => {

            if task.status == a.status {
                (StatusCode::CONFLICT, "Task already of this status").into_response()
            }
            else {
                a.status = task.status.clone();
                info!("Task status changed: {:?}", a);
                let template = TaskTemplate { task: &a };
                Html(template.render().unwrap()).into_response()
            }
        }
    }
}

pub async fn get_task_info_handle(Form(task): Form<TaskIdForm>) -> impl IntoResponse {
    trace!("get_task_info {:?}", task);

    match TASKS.lock().unwrap().get_mut(&task.id) {
        None => {
            warn!("No task found: {:?}", task.id);
            (StatusCode::NOT_FOUND, "Task not found").into_response()
        }
        Some(a) => {
            info!("Task info: {:?}", a);
            let template = TaskModalTemplate { task: &a };
            Html(template.render().unwrap()).into_response()
        }
    }
}

pub async fn set_task_description_handle(Form(task): Form<TaskDescriptionForm>) -> impl IntoResponse {
    trace!("set_task_description {:?}", task);

    for (_, task) in TASKS.lock().unwrap().iter() {
        debug!("{:?}", task)
    }
    match TASKS.lock().unwrap().get_mut(&task.id) {
        None => {
            warn!("No task found: {:?}", task.id);
            (StatusCode::NOT_FOUND, "Task not found").into_response()
        }
        Some(a) => {
            a.description = task.description.clone();
            info!("Task description changed: {:?}", a);
            StatusCode::OK.into_response()
        }
    }
}



pub fn get_tasks_by_status(
    tasks_mutex: &Mutex<HashMap<uid_t, Task>>,
    status: TaskStatus,
) -> Vec<Task> {
    let tasks = tasks_mutex
        .lock()
        .unwrap()
        .iter()
        .filter(|(_, task)| task.status == status)
        .map(|(_, task)| task.clone())
        .collect();
    tasks
}

pub fn get_tasks(tasks_mutex: &Mutex<HashMap<uid_t, Task>>) -> Vec<Task> {
    let tasks = tasks_mutex
        .lock()
        .unwrap()
        .iter()
        .map(|(_, task)| task.clone())
        .collect();
    tasks
}
