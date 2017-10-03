use std::collections::HashMap;
use std::marker::PhantomData;

use rocket::{Outcome, State};
use rocket::request::{self, Request, FromRequest};
use serde::Serialize;

use metadata::Metadata;
use navigation::{MenuBuilder, Link, EMPTY_MENU};

#[derive(Debug, Serialize)]
pub struct TemplateContext<'s, T: Serialize> {
    meta: &'s Metadata,
    menus: HashMap<String, Vec<Link>>,
    data: T,
}

// TODO add dynamically loadable data to TemplateBuilder (could also be done in route when needed)
// e.g. for sidebar

pub struct ContextBuilder<'s, T: Serialize> {
    meta: &'s Metadata,
    menu_builders: HashMap<String, MenuBuilder<'s>>,
    data: PhantomData<*const T>,
}

impl<'a, 'r, T: Serialize> FromRequest<'a, 'r> for ContextBuilder<'a, T> {
    type Error = ();

    fn from_request(request: &'a Request<'r>) -> request::Outcome<Self, Self::Error> {
        let meta = request.guard::<State<Metadata>>()?.inner();
        Outcome::Success(ContextBuilder::new(meta))
    }
}

impl<'s, T: Serialize> ContextBuilder<'s, T> {
    /// Creates a new `TemplateBuilder` from the given [`SatelliteConfig`].
    ///
    /// [`SatelliteConfig`]: struct.SatelliteConfig.html
    pub fn new(meta: &'s Metadata) -> Self {
        ContextBuilder {
            meta,
            menu_builders: HashMap::new(),
            data: PhantomData,
        }
    }

    pub fn prepare_for<P>(&mut self, p: P)
    where
        P: PrepareContextBuilder<T>,
    {
        p.prepare(self);
    }

    pub fn menu_builder(&mut self, key: &str) -> &mut MenuBuilder<'s> {
        let menus = self.meta.menus();
        self.menu_builders.entry(key.to_string()).or_insert_with(
            || {
                let menu: &[Link] = menus.get(key).map(|menu| menu.as_ref()).unwrap_or(
                    &EMPTY_MENU,
                );
                MenuBuilder::new(menu)
            },
        )
    }

    /// Finalizes the Context with the given data.
    pub fn finalize_with_data(mut self, data: T) -> TemplateContext<'s, T> {
        self.add_all_menu_builders();

        let menus = self.menu_builders
            .into_iter()
            .map(|(k, menu)| (k, menu.finalize()))
            .collect();

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

impl<'s, T> ContextBuilder<'s, T>
where
    T: Serialize + Default,
{
    pub fn finalize_with_default(self) -> TemplateContext<'s, T> {
        self.finalize_with_data(T::default())
    }
}

// TODO decide if self needs to be consumed here.
pub trait PrepareContextBuilder<T>
where
    T: Serialize,
{
    fn prepare(self, context_builder: &mut ContextBuilder<T>);
}
