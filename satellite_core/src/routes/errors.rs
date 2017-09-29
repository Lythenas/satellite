use rocket::{self, Catcher};
use rocket::Request;
use rocket_contrib::Template;

use template_builder::TemplateBuilder;

#[error(404)]
fn not_found(req: &Request) -> Template {
    let mut template_builder = req.guard::<TemplateBuilder<()>>().unwrap();

    {
        let main_menu = template_builder.menu_builder("main");
        main_menu.add_class("nav-link");
    }

    template_builder.set_data(());

    // TODO remove unwrap
    template_builder.render("frontend/404").unwrap()
}

pub fn errors() -> Vec<Catcher> {
    errors![not_found]
}