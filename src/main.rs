
#![feature(decl_macro)]

use rocket::get;

fn main() {
    rocket::ignite()
        .mount("/", rocket::routes![hello])
        .launch();
}

#[get("/hello")]
fn hello() -> &'static str {
    "Hello, World"
}
