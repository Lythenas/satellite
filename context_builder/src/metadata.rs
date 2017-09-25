use std::collections::BTreeMap;

use rocket::Config;
use rocket::config::Value;

/// This struct is used to hold meta data for contexts to be passed to [`Template::render`]
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
/// [`Template::render`]: https://api.rocket.rs/rocket_contrib/struct.Template.html#method.render
#[derive(Debug, Serialize, Clone)]
pub struct Metadata {
    title: String,
    description: String,
    authors: Vec<Author>,
}

impl<'a> From<&'a Config> for Metadata {
    /// Checks if the loaded Rocket.toml (stored in [`Config`]) contains a meta table and clones title, description and
    /// authors into a new struct.
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
            Some(&Value::Array(ref authors)) => authors.into_iter().filter_map(|a| match a {
                &Value::String(ref val) => {
                    match Author::try_from(val.clone()) {
                        Ok(author) => Some(author),
                        _ => None,
                    }
                },
                _ => None,
            }).collect(),
            _ => Vec::new(),
        };

        Metadata { title, description, authors }
    }
}

/// Holds the name and email address of one author.
/// Used in [`Metadata`]: struct.Metadata.html
#[derive(Debug, Serialize, Clone, PartialEq)]
pub struct Author {
    name: String,
    email: String,
}

impl Author {
    fn try_from(value: String) -> Result<Self, ()> {
        use regex::Regex;
        let re = Regex::new(r"(.*) <([a-z0-9!#$%&'*+/=?^_`{|}~.-]+@[a-z0-9-]+(\.[a-z0-9-]+)*)>").unwrap();

        let cap = re.captures_iter(&value[..]).next().ok_or(())?;
        let name = cap.get(1).map(|m| m.as_str().to_string()).unwrap_or_default();
        let email = cap.get(2).map(|m| m.as_str().to_string()).unwrap_or_default();

        Ok(Author { name, email })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;

    #[test]
    fn convert_string_to_author() {
        assert_eq!(Ok(Author {
            name: "Random Name".to_string(),
            email: "random@mail.tld".to_string(),
        }), Author::try_from(String::from("Random Name <random@mail.tld>")));

        assert!(Author::try_from(String::from("")).is_err());
        assert!(Author::try_from(String::from("<>")).is_err());
        assert!(Author::try_from(String::from("<random@mail.tld>")).is_err());
        assert!(Author::try_from(String::from("Random Name")).is_err());
    }

    // TODO probably cache Metadata somewhere because it rarely changes

    #[bench]
    fn config_parsing_without_rocket_app(b: &mut Bencher) {
        let config = Config::development().unwrap();

        b.iter(|| {
            Metadata::from(&config)
        });
    }

}