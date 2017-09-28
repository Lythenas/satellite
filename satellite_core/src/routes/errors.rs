use std::collections::HashMap;

use rocket::{self, Catcher, State};
use rocket::Request;
use rocket_contrib::Template;

use metadata::SatelliteConfig;
use navigation::{MenuBuilder, Link};

#[error(404)]
fn not_found(req: &Request) -> Template {
    #[derive(Serialize)]
    struct Context<'a> {
        meta: &'a SatelliteConfig,
        menus: HashMap<String, Vec<Link>>,
    }

    let meta = req.guard::<State<SatelliteConfig>>().unwrap();
    let meta = meta.inner();

    let main_menu = meta.menus().get("main").unwrap();
    let mut builder = MenuBuilder::new(main_menu);
    builder.add_class("nav-link");

    let mut menus = HashMap::new();
    menus.insert("main".to_string(), builder.finalize());

    let context = Context {
        meta,
        menus,
    };

    Template::render("frontend/404", &context)
}

pub fn errors() -> Vec<Catcher> {
    errors![not_found]
}