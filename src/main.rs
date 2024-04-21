mod api;
mod routes;
mod task;

use crate::routes::get_routes;
use crate::task::Task;
use askama::Template;
use log::{info};
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
struct TaskID {
    id: uid_t,
}

#[derive(serde::Deserialize, Debug)]
struct NewTask {
    name: String,
    description: String,
}
