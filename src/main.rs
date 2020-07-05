
#![feature(decl_macro)]
#![feature(try_trait)]

use rocket::get;
use rocket_contrib::json::Json;
use serde::Serialize;

#[macro_use]
extern crate diesel;

mod schema;
mod models;
mod api;

fn main() {
    rocket::ignite()
        .mount("/api/v1", rocket::routes![api::v1::gamerecords::save_gamerecord])
        .launch();
}

#[get("/hello")]
fn hello() -> Json<Greeting> {
    let greeting = Greeting {
        content: "Hello, World".to_string(),
        number: 42
    };

    Json(greeting)
}

#[derive(Serialize)]
struct Greeting {
    content: String,
    number: i32
}
