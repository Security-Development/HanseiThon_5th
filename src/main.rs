#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] 
extern crate rocket;
extern crate rocket_contrib;

use rocket::Rocket;
use rocket_contrib::templates::Template;


mod controllers;
mod database;

fn rocket() -> Rocket {
    return rocket::ignite()
        .manage(database::data_base_connection())
        .mount("/", routes![
            controllers::index, controllers::file, 
            controllers::login, controllers::register,
            controllers::detail
            ])
        .attach(Template::fairing());
}

fn main()
{
    rocket().launch();
}