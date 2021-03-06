use std::collections::HashMap;
use std::marker::PhantomData;
use std::convert::AsRef;

use rocket::{Outcome, State};
use rocket::request::{self, Request, FromRequest, FlashMessage};
use serde::Serialize;

use meta::Metadata;
use nav::{MenuBuilder, Link, EMPTY_MENU};

#[derive(Debug, Serialize)]
pub struct TemplateContext<'s, T: Serialize> {
    meta: &'s Metadata,
    menus: HashMap<String, Vec<Link>>,
    data: T,
    alerts: Vec<Alert>,
}

// TODO add dynamically loadable data to TemplateBuilder (could also be done in route when needed)
// e.g. for sidebar

pub struct ContextBuilder<'s, T: Serialize> {
    meta: &'s Metadata,
    menu_builders: HashMap<String, MenuBuilder>,
    data: PhantomData<*const T>,
    alerts: Vec<Alert>,
}

impl<'a, 'r, T: Serialize> FromRequest<'a, 'r> for ContextBuilder<'a, T> {
    type Error = ();

    fn from_request(request: &'a Request<'r>) -> request::Outcome<Self, Self::Error> {
        let meta = request.guard::<State<Metadata>>()?.inner();
        let flash = request.guard::<Option<FlashMessage>>()?;

        let mut cb = ContextBuilder::new(meta);

        if let Some(flash) = flash {
            cb.add_alert(flash.into());
        }

        Outcome::Success(cb)
    }
}

impl<'s, T: Serialize> ContextBuilder<'s, T> {
    /// Creates a new `TemplateBuilder` from the given [`Metadata`].
    ///
    /// [`Metadata`]: ../metadata/struct.Metadata.html
    pub fn new(meta: &'s Metadata) -> Self {
        ContextBuilder {
            meta,
            menu_builders: HashMap::new(),
            data: PhantomData,
            alerts: Vec::new(),
        }
    }

    /// Returns a mutable reference to the [`MenuBuilder`] with the given key or creates a new one
    /// if it doesn't exist.
    ///
    /// [`MenuBuilder`]: ../navigation/struct.MenuBuilder.html
    ///
    /// # Examples
    ///
    /// ```rust
    /// use context_builder::ContextBuilder;
    /// use context_builder::Metadata;
    ///
    /// let metadata = Metadata::new();
    /// let mut context_builder: ContextBuilder<()> = ContextBuilder::new(&metadata);
    ///
    /// {
    ///     let menu_builder = context_builder.menu_builder("main");
    ///     menu_builder.add_class("menu-item");
    /// }
    ///
    /// let context = context_builder.finalize_with_default();
    /// ```
    pub fn menu_builder(&mut self, key: &str) -> &mut MenuBuilder {
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

    /// Adds an alert to the internal list.
    pub fn add_alert(&mut self, alert: Alert) {
        self.alerts.push(alert);
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
            alerts: self.alerts,
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
    /// Like [`ContextBuilder.finalize_with_data`] but uses the `default` constructor of type `T`.
    /// [`ContextBuilder.finalize_with_data`]: #method.finalize_with_data
    pub fn finalize_with_default(self) -> TemplateContext<'s, T> {
        self.finalize_with_data(T::default())
    }
}

#[derive(Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AlertType {
    Primary,
    Secondary,
    Success,
    Warning,
    #[serde(rename = "danger")]
    Error,
    Info,
    Dark,
    Light,
}

impl<T: AsRef<str>> From<T> for AlertType {
    fn from(name: T) -> AlertType {
        use self::AlertType::*;
        match name.as_ref() {
            "primary" => Primary,
            "secondary" => Secondary,
            "success" => Success,
            "warning" => Warning,
            "error" => Error,
            "info" => Info,
            "dark" => Dark,
            _ => Light,
        }
    }
}

// TODO maybe encapsulate Alert in an enum for easier deserialization
// something like Alert::Inline(AlertData...) or Alert::Cached(key).

#[derive(Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Alert {
    raw: bool,
    #[serde(rename = "type")]
    typ: AlertType,
    header: Option<String>,
    msg: String,
    strong: Option<String>,
    dismissible: bool,
}

impl From<FlashMessage> for Alert {
    fn from(flash: FlashMessage) -> Alert {
        lookup_flash_message(flash)
    }
}

// TODO create a lookup source for static alerts (maybe file or db)
// something like impl From<Cookie> for Alert;

fn lookup_flash_message(flash: FlashMessage) -> Alert {
    Alert {
        raw: false,
        typ: flash.name().into(),
        header: None,
        msg: flash.msg().into(),
        strong: None,
        dismissible: false,
    }
}
