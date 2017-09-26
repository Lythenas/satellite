#![feature(plugin)]
#![plugin(rocket_codegen)]

extern crate rocket;
extern crate rocket_contrib;
#[macro_use]
extern crate serde_derive;

extern crate context_builder;

/// Contains all the routes for Satellite.
mod routes;

use rocket_contrib::Template;
use rocket::fairing::AdHoc;
use rocket::Rocket;

/// Creates a new [`rocket::Rocket`] instance ready to launch the cms.
///
/// You can add more routes, fairings and managed state afert calling
/// this function.
///
/// # Examples
///
/// Launch the instance without additional configuration.
///
/// ```rust,ignore
/// use satellite_core::rocket;
///
/// fn main() {
///     rocket().launch();
/// }
/// ```
///
/// [`rocket::Rocket`]: https://api.rocket.rs/rocket/struct.Rocket.html
pub fn rocket() -> Rocket {
    let rocket = rocket::ignite()
        .attach(Template::fairing())
        .attach(AdHoc::on_attach(|rocket| {
            let config = rocket.config().clone();
            Ok(rocket.manage(config))
        }));

    let rocket = routes::mount_to(rocket);

    rocket
}