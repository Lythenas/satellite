use std::collections::HashMap;
use std::string::ToString;

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
    // TODO make this store a Vec<String> for classes etc. and add a custom serde serializer
    // maybe make attributes: HashMap<String, Attribute>
    // and Attribute is a enum of either Vec<String> for classes, HashMap<String, String> for style
    // or just plain String for everything else.
}

impl Link {
    // TODO remove allow(dead_code) or this method
    #[allow(dead_code)]
    pub fn new<T: ToString, U: ToString>(text: T, url: U) -> Link {
        Link {
            text: text.to_string(),
            url: url.to_string(),
            attributes: HashMap::new(),
        }
    }

    // TODO remove allow(dead_code) or this method
    #[allow(dead_code)]
    pub fn with_attributes<T, U>(text: T, url: U, attributes: HashMap<String, String>) -> Link
    where
        T: ToString,
        U: ToString,
    {
        Link {
            text: text.to_string(),
            url: url.to_string(),
            attributes,
        }
    }

    /// Adds a class to the class attribute string.
    /// Does not check for duplicate classes.
    pub fn add_class(&mut self, class: &str) {
        let classes = self.attributes.entry("class".to_string()).or_insert(
            "".to_string(),
        );
        if !classes.is_empty() {
            classes.push_str(" ");
        }
        classes.push_str(class);
    }

    /// Checks if the link has the given url.
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
pub struct MenuBuilder<'b> {
    menu: Vec<Link>,
    active: Option<&'b str>,
    attributes: HashMap<String, String>,
}

impl<'b> MenuBuilder<'b> {
    pub fn new(menu: &[Link]) -> MenuBuilder {
        MenuBuilder {
            menu: menu.to_owned(),
            active: None,
            attributes: HashMap::new(),
        }
    }

    pub fn add_class(&mut self, class: &str) {
        // TODO don't store classes as string. store them as a set and join it later.
        // this removes duplicates and avoids unnecessary whitespace at beginning without checks.
        let class_attr = self.attributes.entry("class".to_string()).or_insert(
            "".to_string(),
        );
        if !class_attr.is_empty() {
            class_attr.push_str(" ");
        }
        class_attr.push_str(class);
    }

    pub fn set_active(&mut self, url: &'b str) {
        self.active = Some(url);
    }

    pub fn finalize(self) -> Vec<Link> {
        self.menu
            .iter()
            .map(|link| {
                let mut link = link.clone();
                link.extend_attributes(self.attributes.clone());
                add_class_if_active(&mut link, self.active);
                link
            })
            .collect()
    }
}

fn add_class_if_active(link: &mut Link, active: Option<&str>) {
    if let Some(active_url) = active {
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
            }
        );
    }

    #[test]
    fn extend_link_attributes() {
        let mut link = link();
        let mut attributes = HashMap::new();
        attributes.insert(String::from("class"), String::from("active"));
        attributes.insert(String::from("id"), String::from("some-id"));
        link.extend_attributes(attributes);

        assert_eq!(link.attributes, {
            let mut m = HashMap::new();
            m.insert(String::from("class"), String::from("active"));
            m.insert(String::from("id"), String::from("some-id"));
            m
        });
    }

    #[test]
    fn add_class_to_link() {
        let mut link = link();
        link.add_class("active");

        assert_eq!(link.attributes, {
            let mut m = HashMap::new();
            m.insert(String::from("class"), String::from("active"));
            m
        });
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
            let mut attrs = HashMap::new();
            attrs.insert(String::from("class"), String::from("main-nav"));
            Link::with_attributes("Home", "/", attrs)
        });
        assert_eq!(menu[1], {
            let mut attrs = HashMap::new();
            attrs.insert(String::from("class"), String::from("main-nav"));
            Link::with_attributes("About", "/about", attrs)
        });
        assert_eq!(menu[2], {
            let mut attrs = HashMap::new();
            attrs.insert(String::from("class"), String::from("main-nav active"));
            Link::with_attributes("Blog", "/blog", attrs)
        });
        assert_eq!(menu[3], {
            let mut attrs = HashMap::new();
            attrs.insert(String::from("class"), String::from("main-nav"));
            Link::with_attributes("Somewhere else", "/se", attrs)
        });
    }
}
