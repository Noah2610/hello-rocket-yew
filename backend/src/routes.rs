use json::JsonValue;
use maud::{html, Markup};
use rocket::response::NamedFile;
use rocket::Route;
use std::io::{Read, Write};
use std::path::{Path, PathBuf};

pub fn routes() -> Vec<Route> {
    routes![index, static_file, get_todo_data, toggle_todo_entry]
}

#[get("/")]
pub fn index() -> Markup {
    html! {
    // link rel="stylesheet" href="static/styles.css" {}
    body {}
    script src=("/static/hello-rocket-yew-frontend.js") {} }
}

#[get("/static/<path..>")]
pub fn static_file(path: PathBuf) -> Option<NamedFile> {
    NamedFile::open(Path::new("static/").join(path)).ok()
}

#[get("/todo_data")]
pub fn get_todo_data() -> Option<NamedFile> {
    NamedFile::open(Path::new("data/todos.json")).ok()
}

#[get("/toggle_entry/<target_id>")]
pub fn toggle_todo_entry(target_id: usize) -> Option<NamedFile> {
    if let Ok(mut named_file) = NamedFile::open(Path::new("data/todos.json")) {
        let mut json_raw = String::new();
        named_file.read_to_string(&mut json_raw).unwrap();

        let mut json = json::parse(json_raw.as_str()).unwrap();
        for entry in json["entries"].members_mut() {
            let id = entry["id"].as_usize().unwrap();
            if id == target_id {
                entry["completed"] =
                    JsonValue::Boolean(!entry["completed"].as_bool().unwrap());
                break;
            }
        }
        named_file.write_all(json.to_string().as_bytes()).unwrap();

        Some(named_file)
    } else {
        None
    }
}
