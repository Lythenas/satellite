use std::collections::HashMap;
use std::borrow::Cow;

use rocket::{Outcome, State};
use rocket::request::{self, Request, FromRequest};
use rocket_contrib::Template;
use serde::Serialize;

use metadata::SatelliteConfig;
use navigation::{MenuBuilder, Link, EMPTY_MENU};

#[derive(Debug, Serialize)]
pub struct TemplateContext<'s, T: Serialize> {
    meta: &'s SatelliteConfig,
    menus: HashMap<String, Vec<Link>>,
    data: T,
}

// TODO add dynamically loadable data to TemplateBuilder (could also be done in route when needed)
// e.g. for sidebar

pub struct ContextBuilder<'s, T: Serialize> {
    meta: &'s SatelliteConfig,
    menu_builders: HashMap<String, MenuBuilder<'s>>,
    data: Option<T>,
}

impl<'a, 'r, T: Serialize> FromRequest<'a, 'r> for ContextBuilder<'a, T> {
    type Error = ();

    fn from_request(request: &'a Request<'r>) -> request::Outcome<Self, Self::Error> {
        let meta = request.guard::<State<SatelliteConfig>>()?.inner();
        Outcome::Success(ContextBuilder::new(meta))
    }
}

impl<'s, T: Serialize> ContextBuilder<'s, T> {

    /// Creates a new `TemplateBuilder` from the given [`SatelliteConfig`].
    ///
    /// [`SatelliteConfig`]: struct.SatelliteConfig.html
    pub fn new(meta: &'s SatelliteConfig) -> Self {
        ContextBuilder {
            meta,
            menu_builders: HashMap::new(),
            data: None }
    }

    pub fn menu_builder(&mut self, key: &str) -> &mut MenuBuilder<'s> {
        let menus = self.meta.menus();
        self.menu_builders.entry(key.to_string()).or_insert_with(|| {
            let menu: &[Link] = menus.get(key)
                .map(|menu| menu.as_ref()).unwrap_or(&EMPTY_MENU);
            MenuBuilder::new(menu)
        })
    }

    /// Finalizes the Context with the given data.
    pub fn finalize_with_data(mut self, data: T) -> TemplateContext<'s, T> {
        self.add_all_menu_builders();

        let mut menus = HashMap::new();

        for (key, menu) in self.menu_builders.into_iter() {
            menus.insert(key, menu.finalize());
        }

        TemplateContext {
            meta: self.meta,
            menus,
            data,
        }
    }

    fn add_all_menu_builders(&mut self) {
        for key in self.meta.menus().keys() {
            self.menu_builder(key);
        }
    }
}

impl<'s, T> ContextBuilder<'s, T> where T: Serialize + Default {

    pub fn finalize_with_default(self) -> TemplateContext<'s, T> {
        self.finalize_with_data(T::default())
    }

}
