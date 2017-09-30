use std::path::{Path, PathBuf};

use serde::Serialize;
use rocket_contrib::Template;
use rocket::response::NamedFile;
use rocket::Route;

use context_builder::{ContextBuilder, PrepareContextBuilder};

pub fn routes() -> Vec<Route> {
    routes![index, static_files]
}

pub struct Frontend;
impl<T: Serialize> PrepareContextBuilder<T> for Frontend {
    fn prepare(self, context_builder: &mut ContextBuilder<T>) {
        let builder = context_builder.menu_builder("main");
        builder.add_class("nav-link");
    }
}

#[get("/")]
fn index(mut context_builder: ContextBuilder<()>) -> Template {
    // TODO maybe make this part of the Guard.
    // instead of calling prepare_for(SidebarFiller) just use
    // ContextBuilder<(), SidebarFiller> as a guard. Where SiedbarFiller is a managed state.
    context_builder.prepare_for(Frontend);

    {
        let builder = context_builder.menu_builder("main");
        builder.set_active("/");
    }

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
