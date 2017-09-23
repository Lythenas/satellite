//extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate rocket;

use std::collections::BTreeMap;

use rocket::Config;
use rocket::config::Value;

/// Builds a [TemplateContext] to be passed to [rocket_contrib::Template::render].
///
/// # Examples
///
/// Create a [TemplateContext] with only the meta data in Rocket.toml.
/// You need to attach a copy of [rocket::Config] as managed state for this to work.
///
/// ```rust,ignore
/// extern crate rocket;
/// #[macro_use] extern crate rocket_codegen;
/// extern crate rocket_contrib;
///
/// use rocket::{State, Config};
/// use rocket::fairing::AdHoc;
/// use rocket_contrib::Template;
///
/// #[get("/")]
/// fn index(config: State<Config>) -> Template {
///     let context = ContextBuilder::from(*config).finalize();
///     Template::render("index", &context);
/// }
///
/// fn main() {
///     rocket::ignite()
///         .mount("/", routes![index])
///         .attach(Template::fairing())
///         .attach(AdHoc::on_attach(|rocket| {
///             let config = rocket.config().clone();
///             Ok(rocket.manage(config))
///         }))
///         .launch();
/// }
/// ```
#[derive(Debug)]
pub struct ContextBuilder {
    meta: BTreeMap<String, String>,
    extra: BTreeMap<String, String>,
    data: BTreeMap<String, String>,
}

impl<'a> From<&'a Config> for ContextBuilder {
    /// Creates a ContextBuilder with meta data and extras from the given `rocket::Config`.
    fn from(config: &Config) -> Self {
        let meta = match config.get_table("meta") {
            Ok(map) => map.iter().map(&value_to_string_or_default).collect(),
            Err(_) => BTreeMap::new(),
        };

        let extra = match config.get_table("extra") {
            Ok(map) => map.iter().map(&value_to_string_or_default).collect(),
            Err(_) => BTreeMap::new(),
        };

        let data = BTreeMap::new();

        ContextBuilder { meta, extra, data }
    }
}

impl ContextBuilder {
    /// Finalizes the TemplateContext consuming the ContextBuilder.
    pub fn finalize(self) -> TemplateContext {
        TemplateContext {
            meta: self.meta,
            extra: self.extra,
            data: self.data,
        }
    }
}

/// Convenient Context to pass to [rocket_contrib::Template::render].
///
/// Use [ContextBuilder] to build one.
#[derive(Debug, Serialize)]
pub struct TemplateContext {
    meta: BTreeMap<String, String>,
    extra: BTreeMap<String, String>,
    data: BTreeMap<String, String>,
}

/// Helper function for converting the given value to a string if possible or an empty string otherwise.
fn value_to_string_or_default((key, value): (&String, &Value)) -> (String, String) {
    (
        key.clone(),
        value.as_str().map(|s| String::from(s)).unwrap_or_default(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn value_converter_helper() {
        let actual = value_to_string_or_default((&"some_key".into(), &Value::from("some_value")));
        let expected = ("some_key".into(), "some_value".into());

        assert_eq!(actual, expected);
    }
}
