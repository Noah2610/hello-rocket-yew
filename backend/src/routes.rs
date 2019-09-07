use maud::{html, Markup};
use rocket::response::NamedFile;
use rocket::Route;
use std::path::{Path, PathBuf};

pub fn routes() -> Vec<Route> {
    routes![index, static_file]
}

#[get("/")]
pub fn index() -> Markup {
    html! {
        // link rel="stylesheet" href="static/styles.css" {}
        body {}
        script src=("/static/hello-rocket-yew-frontend.js") {}
    }
}

#[get("/static/<path..>")]
pub fn static_file(path: PathBuf) -> Option<NamedFile> {
    NamedFile::open(Path::new("static/").join(path)).ok()
}
