use std::fs::File;
use std::io::prelude::*;
use std::str::FromStr;
use std::fmt;
use std::collections::HashMap;

use rocket::fairing::AdHoc;
use toml;
use serde::de::{self, Deserialize, Deserializer, Visitor};

use nav::Link;

/// This struct is used to hold meta data for contexts to be passed to [`Template::render`]
///
/// [`Template::render`]: https://api.rocket.rs/rocket_contrib/struct.Template.html#method.render
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct Metadata {
    title: String,
    description: String,
    authors: Vec<Author>,
    #[serde(default)]
    sidebar: HashMap<String, SidebarItem>,
    #[serde(default)]
    menus: HashMap<String, Vec<Link>>,
    // TODO add more config fields
}

impl Metadata {
    pub fn fairing() -> AdHoc {
        AdHoc::on_attach(|rocket| {
            let mut input = String::new();
            File::open("Satellite.toml")
                .and_then(|mut f| f.read_to_string(&mut input))
                .unwrap();

            let metadata: Result<Metadata, _> = toml::from_str(input.as_str());

            match metadata {
                Ok(metadata) => Ok(rocket.manage(metadata)),
                Err(e) => {
                    println!("{}", e);
                    Err(rocket)
                }
            }
        })
    }

    /// Creates a new blanket instance of `Metadata`.
    /// All fields will be empty Strings, Vecs and HashMaps.
    pub fn new() -> Metadata {
        Metadata {
            title: String::new(),
            description: String::new(),
            authors: Vec::new(),
            sidebar: HashMap::new(),
            menus: HashMap::new(),
        }
    }

    /// Getter for `Metadata.title`.
    pub fn title(&self) -> &str {
        self.title.as_ref()
    }

    /// Getter for `Metadata.description`.
    pub fn description(&self) -> &str {
        self.description.as_ref()
    }

    /// Getter for `Metadata.authors`.
    pub fn authors(&self) -> &[Author] {
        self.authors.as_ref()
    }

    /// Getter for `Metadata.sidebar`.
    pub fn sidebar(&self) -> &HashMap<String, SidebarItem> {
        &self.sidebar
    }

    /// Getter for `Metadata.menus`.
    pub fn menus(&self) -> &HashMap<String, Vec<Link>> {
        &self.menus
    }
}

/// Holds the name and email address of one author.
/// Used by [`Metadata`]
/// [`Metadata`]: struct.Metadata.html
#[derive(Debug, Serialize, Clone, PartialEq)]
pub struct Author {
    name: String,
    email: String,
}

impl FromStr for Author {
    type Err = AuthorParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use regex::Regex;
        let re = Regex::new(
            r"(.*) <([a-z0-9!#$%&'*+/=?^_`{|}~.-]+@[a-z0-9-]+(\.[a-z0-9-]+)*)>",
        ).unwrap();

        let cap = re.captures_iter(s).next().ok_or(AuthorParseError)?;
        let name = cap.get(1)
            .map(|m| m.as_str().to_string())
            .unwrap_or_default();
        let email = cap.get(2)
            .map(|m| m.as_str().to_string())
            .unwrap_or_default();

        Ok(Author { name, email })
    }
}

impl<'de> Deserialize<'de> for Author {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct AuthorStringVisitor;

        impl<'de> Visitor<'de> for AuthorStringVisitor {
            type Value = Author;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                write!(
                    formatter,
                    "a string in the format 'Name <email@example.com>'"
                )
            }

            fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Author::from_str(v).map_err(|_| {
                    E::custom(format!(
                        "'{}' is not in the format 'Name <email@example.com>'",
                        v
                    ))
                })
            }
        }

        deserializer.deserialize_str(AuthorStringVisitor)
    }
}

/// Represents a parse error for Author.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AuthorParseError;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type", content = "content", rename_all = "snake_case")]
pub enum SidebarItem {
    Text(String),
    TextInset(String),
    Links(Vec<Link>),
    // TODO add dynamically loaded sidebar item
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn metadata() {
        let meta = Metadata::new();
        assert!(meta.title().is_empty());
        assert!(meta.description().is_empty());
        assert!(meta.authors().is_empty());
        assert!(meta.sidebar().is_empty());
        assert!(meta.menus().is_empty());
    }

    #[test]
    fn deserialize_metadata() {
        let data = r#"
            title = "Some Title"
            description = "Some description"
            authors = [
                "Name <email@author.com>",
                "Another <another@author.net>"
            ]
        "#;

        let meta: Metadata = toml::from_str(data).unwrap();

        assert_eq!(
            meta,
            Metadata {
                title: "Some Title".to_string(),
                description: "Some description".to_string(),
                authors: vec![
                    Author {
                        name: "Name".to_string(),
                        email: "email@author.com".to_string(),
                    },
                    Author {
                        name: "Another".to_string(),
                        email: "another@author.net".to_string(),
                    },
                ],
                sidebar: HashMap::new(),
                menus: HashMap::new(),
            }
        );
    }

    #[test]
    fn deserialize_author() {
        assert_eq!(
            Ok(Author {
                name: "Random Name".to_string(),
                email: "random@mail.tld".to_string(),
            }),
            "Random Name <random@mail.tld>".parse()
        );

        assert!("".parse::<Author>().is_err());
        assert!("<>".parse::<Author>().is_err());
        assert!("<random@mail.tld>".parse::<Author>().is_err());
        assert!("Random Name".parse::<Author>().is_err());
    }

    #[test]
    fn deserialize_sidebar_item() {
        let data = r#"
            type = "text"
            content = """Cupcake ipsum dolor sit amet sugar plum. Cheesecake chocolate lemon drops. \
            I love chupa chups chocolate cake lollipop I love cheesecake."""
        "#;
        let sidebar_item: SidebarItem = toml::from_str(data).unwrap();

        assert_eq!(
            sidebar_item,
            SidebarItem::Text(String::from(
                "Cupcake ipsum dolor sit amet sugar plum. \
                   Cheesecake chocolate lemon drops. I love chupa chups chocolate cake lollipop \
                   I love cheesecake.",
            ))
        );

        let data = r#"
            type = "text_inset"
            content = """Cupcake ipsum dolor sit amet sugar plum. Cheesecake chocolate lemon drops. \
            I love chupa chups chocolate cake lollipop I love cheesecake."""
        "#;
        let sidebar_item: SidebarItem = toml::from_str(data).unwrap();

        assert_eq!(
            sidebar_item,
            SidebarItem::TextInset(String::from(
                "Cupcake ipsum dolor sit amet sugar plum. \
                   Cheesecake chocolate lemon drops. I love chupa chups chocolate cake lollipop \
                   I love cheesecake.",
            ))
        );

        let data = r#"
            type = "links"
            content = [
                { text = "Link 1", url = "https://rust-lang.org" },
                { text = "Link 2", url = "https://crates.io" },
            ]
        "#;
        let sidebar_item: SidebarItem = toml::from_str(data).unwrap();

        assert_eq!(
            sidebar_item,
            SidebarItem::Links(vec![
                Link::new("Link 1", "https://rust-lang.org"),
                Link::new("Link 2", "https://crates.io"),
            ])
        );

    }

}
