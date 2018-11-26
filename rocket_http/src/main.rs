#![feature(proc_macro_hygiene, decl_macro)]

use std::collections::HashMap;

#[macro_use] extern crate rocket;
extern crate rocket_contrib;

use rocket::http::RawStr;
use rocket::request::Form;

use rocket_contrib::serve::StaticFiles;
use rocket_contrib::templates::Template;

#[get("/")]
fn index() -> Template {
    let context: HashMap<&str, &str> = [("title", "Hello Templated Rocket!")].iter().cloned().collect();

    Template::render("index", context)
}

#[get("/query?<title>")]
fn query(title: &RawStr) -> String {
    format!("Hello Rocket, {}!", title.as_str())
}

#[get("/tasks/new")]
fn task_new() -> Template {
    Template::render("form", {})
}

#[derive(FromForm)]
struct Task {
    description: String,
    done: bool,
}

#[post("/tasks", data="<task>")]
fn task_create(task: Form<Task>) -> String {
    format!("Task: {}, Done: {}", task.description, task.done)
}

fn routes() -> Vec<rocket::Route> {
    routes![
        index,
        query,
        task_create,
        task_new
    ]
}

fn rocket() -> rocket::Rocket {
    rocket::ignite()
        .mount("/public", StaticFiles::from("public"))
        .mount("/", routes())
        .attach(Template::fairing())
}
fn main() {
    rocket().launch();
}
