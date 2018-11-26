#![feature(proc_macro_hygiene, decl_macro)]

use std::collections::HashMap;

#[macro_use] extern crate rocket;
extern crate rocket_contrib;

use rocket_contrib::serve::StaticFiles;
use rocket_contrib::templates::Template;

#[get("/")]
fn index() -> Template {
    let context: HashMap<&str, &str> = [("title", "Hello Templated Rocket!")].iter().cloned().collect();

    Template::render("index", context)
}

fn rocket() -> rocket::Rocket {
    rocket::ignite()
        .mount("/public", StaticFiles::from("public"))
        .mount("/", routes![index])
        .attach(Template::fairing())
}
fn main() {
    rocket().launch();
}
