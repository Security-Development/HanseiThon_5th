
use std::collections::HashMap;
use rocket_contrib::templates::Template;
use std::path::{Path, PathBuf};
use rocket::response::NamedFile;   




#[get("/static/<file..>")]
pub fn file(file: PathBuf) -> Option<NamedFile> {
    NamedFile::open(Path::new("static/").join(file)).ok()
}

#[get("/")]
pub fn index() -> Template {
    return Template::render("index", HashMap::<String, String>::new());
}

#[get("/login")]
pub fn login() -> Template {
    return Template::render("login", HashMap::<String, String>::new());
}

#[get("/register")]
pub fn register() -> Template {
    return Template::render("register", HashMap::<String, String>::new());
}


#[get("/detail?<index>")]
pub fn detail(index: i32) -> Template {
    let mut data = HashMap::<String, i32>::new();
    data.insert("index".to_string(), index);

    return Template::render("detail", data);
}
