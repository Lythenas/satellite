use rocket::{self, Catcher};
use rocket::Request;
use rocket_contrib::Template;

use context_builder::ContextBuilder;
use routes::frontend;

#[error(404)]
fn not_found(req: &Request) -> Template {
    let mut context_builder = req.guard::<ContextBuilder<()>>().unwrap();
    frontend::prepare_context_builder(None, &mut context_builder);

    let context = context_builder.finalize_with_default();

    Template::render("frontend/404", &context)
}

pub fn errors() -> Vec<Catcher> {
    errors![not_found]
}
