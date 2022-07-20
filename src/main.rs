#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] 
extern crate rocket;
extern crate rocket_contrib;

use std::collections::HashMap;
use rocket::Rocket;
use rocket_contrib::templates::Template;

#[get("/")]
fn index() -> Template {
    return Template::render("index", HashMap::<String, String>::new());
}

#[get("/test?<name>")]
fn test(name: String) -> Template {
    let mut data = HashMap::<String, String>::new();
    data.insert("name".to_string(), name.to_string());
    return Template::render("test", data);
}

fn main()
{
    let app: Rocket = rocket::ignite()
        .mount("/", routes![index])
        .mount("/", routes![test])
        .attach(Template::fairing());

    app.launch();
}