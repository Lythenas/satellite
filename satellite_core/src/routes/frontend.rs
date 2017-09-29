use std::path::{Path, PathBuf};

use rocket_contrib::Template;
use rocket::response::NamedFile;
use rocket::Route;

use template_builder::TemplateBuilder;

pub fn routes() -> Vec<Route> {
    routes![index, static_files]
}

#[get("/")]
fn index(mut template_builder: TemplateBuilder<()>) -> Template {
    {
        let main_menu = template_builder.menu_builder("main");
        main_menu.add_class("nav-link");
        main_menu.set_active("/");
    }

    template_builder.set_data(());

    // TODO make this not use unwrap
    template_builder.render("frontend/index").unwrap()
}

// TODO add more routes

/// Serving static files in `static/` directory before 404ing.
/// This is automatically protected from requesting files outside of the `static/` directory.
#[get("/<path..>", rank = 1000)]
fn static_files(path: PathBuf) -> Option<NamedFile> {
    NamedFile::open(Path::new("static/").join(path)).ok()
}
