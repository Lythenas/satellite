use std::string::ToString;
use std::collections::{HashMap, HashSet};

pub static EMPTY_MENU: [Link; 0] = [];

/// Stores information for a link.
///
/// [`NavigationItem`]: enum.NavigationItem.html
/// [`SidebarItem::Links`]: enum.SidebarItem.html#Links.v
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Link {
    text: String,
    url: String,
    #[serde(default)]
    attributes: HashMap<String, String>,
    #[serde(default)]
    classes: HashSet<String>,
}

impl Link {
    /// Creates a new `Link` with the given `text` and `url`.
    pub fn new<T: ToString, U: ToString>(text: T, url: U) -> Link {
        Link {
            text: text.to_string(),
            url: url.to_string(),
            attributes: HashMap::new(),
            classes: HashSet::new(),
        }
    }

    /// Adds a class to this link.
    /// Duplicate classes are ignored.
    pub fn add_class<T: ToString>(&mut self, class: T) {
        self.classes.insert(class.to_string());
    }

    /// Adds classes to this link.
    /// Duplicate classes are ignored.
    pub fn add_classes<I, T>(&mut self, classes: I)
    where I: IntoIterator<Item=T>, T: ToString {
        for class in classes.into_iter() {
            self.add_class(class);
        }
    }

    /// Returns a reference to the url.
    pub fn url(&self) -> &str {
        self.url.as_str()
    }

    pub fn extend_attributes(&mut self, new_attributes: HashMap<String, String>) {
        for (attr, val) in new_attributes {
            let current = self.attributes.entry(attr).or_insert("".to_string());
            if !current.is_empty() {
                current.push_str(" ");
            }
            current.push_str(val.as_str());
        }
    }
}

/// Builder for `Vec<Link>`.
/// [`Link`](struct.Link.html)
pub struct MenuBuilder {
    menu: Vec<Link>,
    active: Option<String>,
    attributes: HashMap<String, String>,
    classes: HashSet<String>,
}

impl MenuBuilder {
    pub fn new(menu: &[Link]) -> MenuBuilder {
        MenuBuilder {
            menu: menu.to_owned(),
            active: None,
            attributes: HashMap::new(),
            classes: HashSet::new(),
        }
    }

    /// Adds a class to all menu items.
    pub fn add_class<T: ToString>(&mut self, class: T) {
        self.classes.insert(class.to_string());
    }

    /// Sets the currently active url.
    pub fn set_active<T: ToString>(&mut self, url: T) {
        self.active = Some(url.to_string());
    }

    /// Finalizes the menu and returns a `Vec<Link>`.
    pub fn finalize(self) -> Vec<Link> {
        self.menu
            .iter()
            .map(|link| {
                let mut link = link.clone();
                link.extend_attributes(self.attributes.clone());
                link.add_classes(self.classes.clone());
                add_class_if_active(&mut link, &self.active);
                link
            })
            .collect()
    }
}

fn add_class_if_active(link: &mut Link, active: &Option<String>) {
    if let Some(ref active_url) = *active {
        if link.url() == active_url {
            link.add_class("active");
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn link() -> Link {
        Link::new("Click here", "https://somewhere.net")
    }

    #[test]
    fn create_link() {
        let link = link();
        assert_eq!(
            link,
            Link {
                text: String::from("Click here"),
                url: String::from("https://somewhere.net"),
                attributes: HashMap::new(),
                classes: HashSet::new(),
            }
        );
    }

    #[test]
    fn extend_link_attributes() {
        let mut link = link();
        let attributes: HashMap<String, String> = convert_args!(hashmap!(
            "class" => "active",
            "id" => "some-id",
        ));
        link.extend_attributes(attributes);

        assert_eq!(link.attributes, convert_args!(hashmap!(
            "class" => "active",
            "id" => "some-id",
        )));
    }

    #[test]
    fn add_class_to_link() {
        let mut link = link();
        link.add_class("active");

        assert_eq!(link.classes, convert_args!(hashset!(
            "active"
        )));
    }

    #[test]
    fn build_menu() {
        let links = vec![
            Link::new("Home", "/"),
            Link::new("About", "/about"),
            Link::new("Blog", "/blog"),
            Link::new("Somewhere else", "/se"),
        ];
        let mut builder = MenuBuilder::new(links.as_ref());
        builder.add_class("main-nav");
        builder.set_active("/blog");
        let menu = builder.finalize();

        assert_eq!(menu[0], {
            Link {
                text: "Home".to_string(),
                url: "/".to_string(),
                attributes: HashMap::new(),
                classes: convert_args!(hashset!(
                    "main-nav"
                )),
            }
        });
        assert_eq!(menu[1], {
            Link {
                text: "About".to_string(),
                url: "/about".to_string(),
                attributes: HashMap::new(),
                classes: convert_args!(hashset!(
                    "main-nav"
                )),
            }
        });
        assert_eq!(menu[2], {
            Link {
                text: "Blog".to_string(),
                url: "/blog".to_string(),
                attributes: HashMap::new(),
                classes: convert_args!(hashset!(
                    "main-nav",
                    "active",
                )),
            }
        });
        assert_eq!(menu[3], {
            Link {
                text: "Somewhere else".to_string(),
                url: "/se".to_string(),
                attributes: HashMap::new(),
                classes: convert_args!(hashset!(
                    "main-nav"
                )),
            }
        });
    }
}
