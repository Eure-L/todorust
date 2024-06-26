mod api;
mod routes;
mod task;

use crate::routes::get_routes;
use crate::task::{Task, TaskStatus};
use askama::Template;
use log::info;
use tokio::net::unix::uid_t;

#[derive(Template)]
#[template(path = "index.html", ext = "html")]
struct IndexTemplate<'a> {
    tasks: &'a Vec<Task>,
}

#[derive(Template)]
#[template(path = "tasks.html", ext = "html")]
struct TasksTemplate<'a> {
    tasks: &'a Vec<Task>,
}

#[derive(Template)]
#[template(path = "task.html", ext = "html")]
struct TaskTemplate<'a> {
    task: &'a Task,
}

#[derive(Template)]
#[template(path = "task-modal.html", ext = "html")]
struct TaskModalTemplate<'a> {
    task: &'a Task,
}

#[tokio::main]
async fn main() {
    env_logger::init();

    info!("Starting Server");

    // build our application with a route
    let app = get_routes();

    // run it with hyper on localhost:3000
    axum_server::Server::bind("127.0.0.1:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}

#[derive(serde::Deserialize, Debug)]
struct TaskIdForm {
    id: uid_t,

}

#[derive(serde::Deserialize, Debug)]
struct TaskDescriptionForm {
    id: uid_t,
    description: String,

}

#[derive(serde::Deserialize, Debug)]
struct TaskStatusForm {
    id: uid_t,
    status: TaskStatus,
}

#[derive(serde::Deserialize, Debug)]
struct NewTaskForm {
    #[serde(default = "default_name")]
    name: String,
    #[serde(default = "default_description")]
    description: String,
}

fn default_name() -> String {
    String::from("<Task>")
}
fn default_description() -> String {
    String::from("<Task Description>")
}