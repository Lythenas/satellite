#![feature(plugin)]
#![plugin(rocket_codegen)]

extern crate rocket;
extern crate rocket_contrib;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;
extern crate regex;
extern crate toml;

/// Contains all the routes for Satellite.
/// (private)
mod routes;

/// Contains Metadata structs for use in routes.
pub mod metadata;

/// Contains helpers for building menus and links.
pub mod navigation;

/// Contains [`ContextBuilder`].
/// Which is a useful guard for all routes that return a [`Template`].
///
/// [`TemplateBuilder`]: struct.TemplateBuilder.html
/// [`Template`]: https://api.rocket.rs/rocket_contrib/struct.Template.html
pub mod context_builder;

use rocket_contrib::Template;
use rocket::Rocket;

// TODO rethink how to use this module/library
pub use metadata::Metadata;
pub use context_builder::ContextBuilder;

/// Creates a new [`rocket::Rocket`] instance ready to launch the cms.
///
/// You can add more routes, fairings and managed state afert calling
/// this function.
///
/// # Examples
///
/// See https://github.com/Lythenas/satellite/blob/master/src/main.rs
///
/// [`rocket::Rocket`]: https://api.rocket.rs/rocket/struct.Rocket.html
pub fn rocket() -> Rocket {
    let rocket = rocket::ignite() // _
        .attach(Template::fairing())
        .attach(Metadata::fairing());

    let rocket = routes::mount_to(rocket);
    let rocket = routes::add_catchers_to(rocket);

    rocket
}
