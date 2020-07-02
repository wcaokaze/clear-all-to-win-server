
#![feature(decl_macro)]

use rocket::get;
use rocket_contrib::json::Json;
use serde::Serialize;

#[macro_use]
extern crate diesel;

mod schema;
mod models;

fn main() {
    rocket::ignite()
        .mount("/", rocket::routes![hello])
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
