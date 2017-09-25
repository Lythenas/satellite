use std::path::{Path, PathBuf};
use std::collections::HashMap;

use rocket_contrib::Template;
use rocket::response::NamedFile;
use rocket::Route;
use rocket::State;
use rocket::Config;

use context_builder::Metadata;

#[get("/")]
fn index(config: State<Config>) -> Template {
    // TODO refactor
    #[derive(Serialize)]
    struct IndexContext {
        meta: Metadata,
        extra: HashMap<String, String>,
        data: Vec<String>,
    }

    let context = IndexContext {
        meta: Metadata::with_config(config.inner()),
        extra: HashMap::new(),
        data: vec![],
    };

    Template::render("index", &context)
}

/// Serving static files in `static/` directory before 404ing.
/// This is automatically protected from requesting files outside of the `static/` directory.
#[get("/<path..>", rank = 1000)]
fn static_files(path: PathBuf) -> Option<NamedFile> {
    NamedFile::open(Path::new("static/").join(path)).ok()
}

/// Returns a [`Vec`]`<`[`Route`]`>` of all routes in satellite.
///
/// # Usage
///
/// ```rust,ignore
/// fn main() {
///     rocket::ignite()
///         .mount("/", routes::routes())
///         .launch();
/// }
/// ```
///
/// [`Vec`]: https://doc.rust-lang.org/std/vec/struct.Vec.html
/// [`Route`]: https://api.rocket.rs/rocket/struct.Route.html
pub fn routes() -> Vec<Route> {
    routes![index, static_files]
}
