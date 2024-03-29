#![feature(proc_macro_hygiene)]
#![feature(decl_macro)]

#[macro_use]
extern crate rocket;

// GETがきたときに"Hello, world!"というレスポンスを返す
#[get("/")]
fn index() -> &'static str {
    "[GET] Hello, World!"
}

fn main() {
    rocket::ignite()
        .mount("/", routes![index])
        .launch();
}