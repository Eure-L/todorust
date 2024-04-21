use askama::Template;
use axum::{
    extract::{Form},
    response::Html,
    routing::{delete, get, post},
    Router,
};
use lazy_static::lazy_static;
use log::{info, trace, warn};
use std::sync::Mutex;
use tokio::io::AsyncReadExt;

lazy_static! {
    static ref TASKS: Mutex<Vec<String>> = Mutex::new(vec![]);
}

#[derive(Template)]
#[template(path = "index.html", ext = "html")]
struct IndexTemplate<'a> {
    tasks: &'a Vec<String>,
}

#[derive(Template)]
#[template(path = "tasks.html", ext = "html")]
struct TasksTemplate<'a> {
    tasks: &'a Vec<String>,
}

#[tokio::main]
async fn main() {
    env_logger::init();
    info!("Starting Server");

    // build our application with a route
    let app = Router::new()
        .route("/", get(index))
        .route("/tasks/add", post(add_task_handle))
        .route("/tasks/delete", delete(delete_task_handle));

    // run it with hyper on localhost:3000
    axum_server::Server::bind("127.0.0.1:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn index() -> Html<String> {
    trace!("index");
    let tasks = TASKS.lock().unwrap();
    let template = IndexTemplate { tasks: &tasks };
    Html(template.render().unwrap())
}

async fn add_task_handle(Form(input): Form<Task>) -> Html<String> {
    let new_task = input.task;
    trace!("add_task {new_task}");
    let mut tasks = TASKS.lock().unwrap();
    tasks.push(new_task.clone());

    let template = TasksTemplate { tasks: &tasks };
    Html(template.render().unwrap())
}

async fn delete_task_handle(Form(input): Form<Task>) -> Html<String> {
    trace!("{:?}",input);
    let task_del = input.task;
    trace!("delete_task {task_del}");

    let mut tasks = TASKS.lock().unwrap();

    if let Some(index) = tasks.iter().position(|t| t == &task_del) {
        tasks.remove(index);
    } else {
        warn!("Task not found: {task_del}");
    }

    let template = TasksTemplate { tasks: &tasks };
    Html(template.render().unwrap())
}

#[derive(serde::Deserialize, Debug)]
struct Task {
    task: String,
}