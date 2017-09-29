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

pub struct TemplateBuilder<'s, T: Serialize> {
    meta: &'s SatelliteConfig,
    menus: HashMap<String, MenuBuilder<'s>>,
    data: Option<T>,
}

impl<'a, 'r, T: Serialize> FromRequest<'a, 'r> for TemplateBuilder<'a, T> {
    type Error = ();

    fn from_request(request: &'a Request<'r>) -> request::Outcome<Self, Self::Error> {
        let meta = request.guard::<State<SatelliteConfig>>()?.inner();
        Outcome::Success(TemplateBuilder::new(meta))
    }
}

impl<'s, T: Serialize> TemplateBuilder<'s, T> {

    /// Creates a new `TemplateBuilder` from the given [`SatelliteConfig`].
    ///
    /// [`SatelliteConfig`]: struct.SatelliteConfig.html
    pub fn new(meta: &'s SatelliteConfig) -> Self {
        TemplateBuilder {
            meta,
            menus: HashMap::new(),
            data: None }
    }

    pub fn menu_builder(&mut self, key: &str) -> &mut MenuBuilder<'s> {
        let menus = self.meta.menus();
        self.menus.entry(key.to_string()).or_insert_with(|| {
            let menu: &[Link] = menus.get(key)
                .map(|menu| menu.as_ref()).unwrap_or(&EMPTY_MENU);
            MenuBuilder::new(menu)
        })
    }

    pub fn set_data(&mut self, data: T) {
        self.data = Some(data);
    }

    pub fn render<N>(self, name: N) -> Result<Template, DataFieldRequiredError>
    where N: Into<Cow<'static, str>> {
        let context = self.into_context()?;
        Ok(Template::render(name, &context))
    }

    fn into_context(mut self) -> Result<TemplateContext<'s, T>, DataFieldRequiredError> {
        self.add_all_menu_builders();

        let mut menus = HashMap::new();

        for (key, menu) in self.menus.into_iter() {
            menus.insert(key, menu.finalize());
        }

        match self.data {
            Some(data) => Ok(TemplateContext {
                meta: self.meta,
                menus,
                data,
            }),
            None => Err(DataFieldRequiredError),
        }
    }

    fn add_all_menu_builders(&mut self) {
        for key in self.meta.menus().keys() {
            self.menu_builder(key);
        }
    }
}

// TODO make this a real error
#[derive(Debug)]
pub struct DataFieldRequiredError;