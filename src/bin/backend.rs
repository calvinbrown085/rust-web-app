#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;
#[macro_use]
extern crate rocket_contrib;
#[macro_use]
extern crate serde;

use rust_web_app::db::{query_task, establish_connection};
use rust_web_app::db::models::*;
use rocket_contrib::json::Json;

#[get("/tasks")]
fn tasks_get() -> Json<JsonApiResponse> {
    let mut response = JsonApiResponse { data: vec![], };

    let conn = establish_connection();
    for task in query_task(&conn) {
        response.data.push(task);
    }

    Json(response)
}

fn main() {
    rocket::ignite()
        .mount("/", routes![tasks_get])
        .launch();
}


#[derive(Serialize)]
struct JsonApiResponse {
    data: Vec<Task>,
}

