#![feature(decl_macro)]
#![feature(proc_macro_hygiene)]

extern crate json;
#[macro_use]
extern crate rocket;
#[macro_use]
extern crate maud;

mod routes;

fn main() {
    launch();
}

fn launch() {
    rocket::ignite().mount("/", routes::routes()).launch();
}
