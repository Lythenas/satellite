pub mod frontend;
pub mod backend;
pub mod errors;

use rocket::Rocket;

/// Mounts all routes provided by this crate to the given [`Rocket`] instance and returns it.
/// Only used in [`satellite_core::rocket`]
///
/// [`Rocket`]: https://api.rocket.rs/rocket/struct.Rocket.html
/// [`satellite_core::rocket`]: fn.rocket.html
pub fn mount_to(rocket: Rocket) -> Rocket {
    rocket.mount("/", frontend::routes()).mount(
        "/admin",
        backend::routes(),
    )
}

pub fn add_catchers_to(rocket: Rocket) -> Rocket {
    rocket.catch(errors::errors())
}
