mod router;

use askama::Template;
use axum::{
    extract::{Form},
    response::Html,
};
use log::{info, trace, warn};
use std::sync::Mutex;
use crate::router::get_router;
use std::collections::HashMap;
use std::fmt::{Display, Formatter};
use lazy_static::lazy_static;
use tokio::net::unix::uid_t;
use uuid::Uuid;


lazy_static! {
    static ref TASKS: Mutex<HashMap<uid_t, Task>> = Mutex::new(HashMap::new());
}

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

#[tokio::main]
async fn main() {
    env_logger::init();
    info!("Starting Server");

    // build our application with a route
    let app = get_router();

    // run it with hyper on localhost:3000
    axum_server::Server::bind("127.0.0.1:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn index() -> Html<String> {
    trace!("index");

    let tasks = TASKS
        .lock()
        .unwrap()
        .iter()
        .map(|(_, task)| { task.clone() })
        .collect();

    let template = IndexTemplate { tasks: &tasks };
    Html(template.render().unwrap())
}

async fn add_task_handle(Form(input): Form<NewTask>) -> Html<String> {
    trace!("add_task {:?}", input);

    let mut tasks = TASKS.lock().unwrap();

    let new_id = Uuid::new_v4().as_u128() as uid_t;
    let new_task = Task {
        name: input.name,
        id: new_id,
        description: input.description,
    };

    trace!("Inerting task");
    tasks.insert(new_id, new_task);
    trace!("Inerting task");

    let new_tasks = tasks
        .iter()
        .map(|(_, task)| { task.clone() })
        .collect();

    trace!("Rendering tasks");
    let template = TasksTemplate { tasks: &new_tasks };
    Html(template.render().unwrap())
}

async fn delete_task_handle(Form(task): Form<TaskID>) -> Html<String> {
    trace!("delete_task {:?}", task);

    let _ = match TASKS.lock().unwrap().remove(&task.id) {//(&task.id) {
        None => {
            warn!("No task found: {:?}", task.id)
        }
        Some(a) => {
            info!("Task deleted: {:?}", a)
        }
    };

    let tasks = TASKS
        .lock()
        .unwrap()
        .iter_mut()
        .map(|(_, task)| { task.clone()})
        .collect();

    let template = TasksTemplate { tasks: &tasks };
    Html(template.render().unwrap())
}

#[derive(serde::Deserialize, Debug, Clone)]
struct Task {
    name: String,
    id: uid_t,
    description: String,
}

#[derive(serde::Deserialize, Debug)]
struct TaskID {
    id: uid_t,
}

#[derive(serde::Deserialize, Debug)]
struct NewTask {
    name: String,
    description: String,
}

impl Display for Task {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} (id: {})", self.name, self.id)
    }
}
