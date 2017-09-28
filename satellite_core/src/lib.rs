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
mod routes;

/// Contains Metadata structs for use in routes.
mod metadata;

/// Contains [`NavigationBuilder´] and other navigation related structs and methods.
///
/// [`NaviagationBuilder`]: struct.NavigationBuilder.html
mod navigation;

use rocket_contrib::Template;
use rocket::Rocket;

use metadata::SatelliteConfig;

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
    let rocket = rocket::ignite()
        .attach(Template::fairing())
        .attach(SatelliteConfig::fairing());

    let rocket = routes::mount_to(rocket);
    let rocket = routes::add_catchers_to(rocket);

    rocket
}