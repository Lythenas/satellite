#![feature(plugin)]
#![plugin(rocket_codegen)]

extern crate rocket;
extern crate rocket_contrib;
extern crate serde;
//#[macro_use]
//extern crate serde_derive;

extern crate context_builder;

/// Contains all the routes
mod routes;

use rocket_contrib::Template;
use context_builder::Metadata;

fn main() {
    // TODO make this more extensible
    let rocket = rocket::ignite() // _
        .attach(Template::fairing())
        .attach(Metadata::fairing());

    // TODO make this better
    let rocket = routes::mount_to(rocket);
    let rocket = routes::add_catchers_to(rocket);

    rocket.launch();
}
