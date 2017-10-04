use std::path::{Path, PathBuf};

use serde::Serialize;
use rocket_contrib::Template;
use rocket::response::NamedFile;
use rocket::Route;

use context_builder::{ContextBuilder};

pub fn routes() -> Vec<Route> {
    routes![index, static_files]
}

pub fn prepare_context_builder<'a, T: Serialize>(current_url: Option<&'a str>, context_builder: &mut ContextBuilder<'a, T>) {
    let builder = context_builder.menu_builder("main");
    builder.add_class("nav-link");
    if let Some(url) = current_url {
        builder.set_active(url);
    }
}

#[get("/")]
fn index(mut context_builder: ContextBuilder<()>) -> Template {
    prepare_context_builder(Some("/"), &mut context_builder);

    let context = context_builder.finalize_with_data(());

    Template::render("frontend/index", &context)
}

// TODO add more routes

/// Serving static files in `static/` directory before 404ing.
/// This is automatically protected from requesting files outside of the `static/` directory.
#[get("/<path..>", rank = 1000)]
fn static_files(path: PathBuf) -> Option<NamedFile> {
    NamedFile::open(Path::new("static/").join(path)).ok()
}
