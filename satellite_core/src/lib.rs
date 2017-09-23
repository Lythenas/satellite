#![feature(plugin)]
#![plugin(rocket_codegen)]

extern crate rocket;
extern crate rocket_contrib;
#[macro_use] extern crate serde_derive;

use rocket_contrib::Template;

#[get("/")]
fn index() -> &'static str {
    "Hello, World!"
}

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
        .mount("/", routes![index])
        .attach(Template::fairing())
}
