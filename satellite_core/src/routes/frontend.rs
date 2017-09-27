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

/// Serving static files in `static/` directory before 404ing.
/// This is automatically protected from requesting files outside of the `static/` directory.
#[get("/<path..>", rank = 1000)]
fn static_files(path: PathBuf) -> Option<NamedFile> {
    NamedFile::open(Path::new("static/").join(path)).ok()
}
