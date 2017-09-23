use std::collections::HashMap;
use std::path::{Path, PathBuf};

use rocket_contrib::Template;
use rocket::response::NamedFile;
use rocket::Route;

pub fn routes() -> Vec<Route> {
    routes![index, static_files]
}

#[get("/")]
fn index() -> Template {
    // TODO create more convenient Context
    let context = {
        let mut map = HashMap::new();
        map.insert("meta_title", "Test Blog");
        map.insert("meta_description", "This is a test blog for Satellite");
        map.insert("meta_author", "Matthias Seiffert");
        map.insert("extra_about", "Some text about the author. Cupcake ipsum dolor sit amet jelly. Cake brownie jujubes jujubes. Brownie tart chocolate bar. Apple pie I love pastry muffin gummi bears I love cheesecake.");
        map
    };
    Template::render("index", &context)
}

/// Serving static files in `static/` directory before 404ing.
/// This is automatically protected from requesting files outside of the `static/` directory.
#[get("/<path..>", rank = 1000)]
fn static_files(path: PathBuf) -> Option<NamedFile> {
    NamedFile::open(Path::new("static/").join(path)).ok()
}
