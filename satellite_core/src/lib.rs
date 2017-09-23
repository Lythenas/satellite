#![feature(plugin)]
#![plugin(rocket_codegen)]

extern crate rocket;
extern crate rocket_contrib;

extern crate context_builder;

/// Contains all the routes for Satellite.
mod routes;

use rocket_contrib::Template;
use rocket::fairing::AdHoc;

/// Creates a new Rocket instance ready to launch the cms.
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
pub fn rocket() -> rocket::Rocket {
    rocket::ignite()
        .mount("/", routes::routes())
        .attach(Template::fairing())
        .attach(AdHoc::on_attach(|rocket| {
            let config = rocket.config().clone();
            Ok(rocket.manage(config))
        }))
}
