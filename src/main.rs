#![feature(plugin)]
#![plugin(rocket_codegen)]

// TODO ?
#![feature(custom_derive)]

extern crate rocket;
extern crate rocket_contrib;
extern crate serde;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate diesel;
#[macro_use]
extern crate diesel_codegen;
extern crate r2d2;
extern crate r2d2_diesel;
extern crate chrono;

extern crate context_builder;

/// Contains helpers for diesel.
mod db;

/// Contains all the controllers.
mod controllers;

/// Contains all the form structs and helpers.
mod forms;

/// Contains all the routes.
mod routes;

use rocket_contrib::Template;
use context_builder::Metadata;

fn main() {
    // TODO make this more extensible
    let rocket = rocket::ignite() // _
        .attach(Template::fairing())
        .attach(Metadata::fairing())
        .manage(db::init_pool());

    // TODO make this better
    let rocket = routes::mount_to(rocket);
    let rocket = routes::add_catchers_to(rocket);

    rocket.launch();
}
