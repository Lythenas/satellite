use std::path::{Path, PathBuf};

use rocket_contrib::Template;
use rocket::response::NamedFile;
use rocket::Route;

use context_builder::ContextBuilder;

pub fn routes() -> Vec<Route> {
    routes![index, static_files]
}

#[get("/")]
fn index(mut template_builder: ContextBuilder<()>) -> Template {
    // TODO make this easier to do.
    // probably just add callbacks to TemplateBuilder that get called when rendering the template
    // something like this:
    // template_builder.build_menu("main", |builder| {
    //      builder.add_class("nav-link");
    //      builder.set_active("/");
    // });
    //
    // TODO maybe also create a global callback that gets called on every route
    // or create different versions of the TemplateBuilder for different page types
    // maybe with another generic: TemplateBuilder<Data, Type>.
    // And Type implements TemplateBuilderModifier with one function.
    {
        let main_menu = template_builder.menu_builder("main");
        main_menu.add_class("nav-link");
        main_menu.set_active("/");
    }

    let context = template_builder.finalize_with_data(());

    Template::render("frontend/index", &context)
}

// TODO add more routes

/// Serving static files in `static/` directory before 404ing.
/// This is automatically protected from requesting files outside of the `static/` directory.
#[get("/<path..>", rank = 1000)]
fn static_files(path: PathBuf) -> Option<NamedFile> {
    NamedFile::open(Path::new("static/").join(path)).ok()
}
