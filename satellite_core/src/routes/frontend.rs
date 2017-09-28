use std::path::{Path, PathBuf};
use std::collections::HashMap;

use rocket_contrib::Template;
use rocket::response::NamedFile;
use rocket::{Route, State};

use metadata::SatelliteConfig;
use navigation::{Link, MenuBuilder};

use serde_json;

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
    #[derive(Serialize, Debug)]
    struct IndexContext<'a> {
        meta: &'a SatelliteConfig,
        menus: HashMap<String, Vec<Link>>,
        data: Vec<String>,
    }

    let meta: &SatelliteConfig = meta.inner();
    let main_menu = meta.menus().get("main").unwrap();
    let mut builder = MenuBuilder::new(main_menu);
    builder.add_class("nav-link");
    builder.set_active("/");

    let mut menus = HashMap::new();
    menus.insert("main".to_string(), builder.finalize());

    let context = IndexContext {
        meta,
        menus,
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
