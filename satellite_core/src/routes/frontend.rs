use std::path::{Path, PathBuf};
//use std::collections::HashMap;

use rocket_contrib::Template;
use rocket::response::NamedFile;
use rocket::{Route, State};

use metadata::SatelliteConfig;
use navigation::Link;

pub fn routes() -> Vec<Route> {
    routes![index, static_files]
}

// TODO add a Template guard instead of State<SatelliteConfig>.
// The guard should take care of setting everything up.
// It loads e.g. the dynamically loaded parts of the sidebar (also TODO).
// We then just pass it the template name and a inner context containing only individual data
// e.g. list of posts for index.

#[get("/")]
fn index(meta: State<SatelliteConfig>) -> Template {
    // TODO refactor
    #[derive(Serialize)]
    struct IndexContext<'a> {
        meta: &'a SatelliteConfig,
        data: Vec<String>,
    }
    let context = IndexContext {
        meta: meta.inner(),
        data: vec![],
    };

    Template::render("frontend/index", &context)
}

// TODO add more routes

/// Serving static files in `static/` directory before 404ing.
/// This is automatically protected from requesting files outside of the `static/` directory.
#[get("/<path..>", rank = 1000)]
fn static_files(path: PathBuf) -> Option<NamedFile> {
    NamedFile::open(Path::new("static/").join(path)).ok()
}
