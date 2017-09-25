//extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate rocket;

use std::collections::BTreeMap;

use rocket::Config;
use rocket::config::Value;

/// This struct is used to hold meta data for contexts to be passed to [`Template::render`]
///
/// [`Template::render`]: https://api.rocket.rs/rocket_contrib/struct.Template.html#method.render
#[derive(Debug, Serialize)]
pub struct MetaData {
    title: String,
    description: String,
    authors: Vec<String>, // TODO split author strings into name and email (own struct)
}

impl<'a> From<&'a Config> for MetaData {
    /// Checks if the loaded Rocket.toml (stored in [`Config`]) contains a meta table and clones title, description and
    /// authors into a new struct.
    ///
    /// # Examples
    ///
    /// ```rust,ignore
    /// #[get("/")]
    /// fn index(config: State<Config>) -> Template {
    ///     let context = IndexContext {
    ///         meta: MetaData::from(config.inner()),
    ///         extra: HashMap::new(),
    ///         data: Vec::new(),
    ///     };
    ///
    ///     Template::render("index", &context)
    /// }
    /// ```
    ///
    /// [`Config`]: https://api.rocket.rs/rocket/struct.Config.html
    fn from(config: &Config) -> Self {
        let meta = match config.get_table("meta") {
            Ok(map) => map.clone(),
            Err(_) => BTreeMap::new(),
        };

        let title = match meta.get("title") {
            Some(&Value::String(ref val)) => val.clone(),
            _ => String::new(),
        };
        let description = match meta.get("description") {
            Some(&Value::String(ref val)) => val.clone(),
            _ => String::new(),
        };
        let authors = match meta.get("authors") {
            Some(&Value::Array(ref authors)) => authors.into_iter().map(|a| match a {
                &Value::String(ref val) => val.clone(),
                _ => String::new(),
            }).collect(),
            _ => Vec::new(),
        };

        MetaData { title, description, authors }
    }
}

// TODO
pub struct NavigationBuilder {

}

#[cfg(test)]
mod tests {
    use super::*;

}
