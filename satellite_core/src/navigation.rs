use std::collections::HashMap;
use std::string::ToString;

// TODO rethink the whole navigation builder thing.
// Is it necessary or can it be done simpler.

/// This type is used to build a navigation menu.
/// It can contain [`Link`]s and other sub `NavigationMenu`s.
#[derive(Debug, Serialize, Clone, PartialEq)]
pub struct NavigationMenu {
    items: Vec<NavigationItem>,
}

impl NavigationMenu {
    /// Creates an empty `NavigationMenu`.
    pub fn new() -> NavigationMenu {
        NavigationMenu {
            items: Vec::new(),
        }
    }

    /// Adds a [`NavigationItem`] to the end of the menu.
    /// Prefer using [`push_link`], [`push_link_to`] for adding links
    /// and [`push_submenu`] for adding submenus.
    ///
    /// [`NavigationItem`]: enum.NavigationItem.html
    /// [`push_link`]: #method.push_link
    /// [`push_link_to`]: #method.push_link_to
    /// [`push_submenu`]: #method.push_submenu
    pub fn push_item(&mut self, item: NavigationItem) -> &mut Self {
        self.items.push(item);
        self
    }

    /// Adds a [`Link`] to the end of the menu.
    ///
    /// [`Link`]: struct.Link.html
    pub fn push_link(&mut self, link: Link) -> &mut Self {
        self.items.push(NavigationItem::Link(link));
        self
    }

    /// Adds a [`Link`] to the given `url` with the given `text` to the end of the menu.
    ///
    /// [`Link`]: struct.Link.html
    pub fn push_link_to<T: ToString, U: ToString>(&mut self, text: T, url: U) -> &mut Self {
        self.items.push(NavigationItem::Link(Link::new(text, url)));
        self
    }

    /// Adds another [`NavigationMenu`] to the end of this menu.
    ///
    /// [`NavigationMenu`]: struct.NavigationMenu.html
    pub fn push_submenu(&mut self, submenu: NavigationMenu) -> &mut Self {
        self.items.push(NavigationItem::SubMenu(submenu));
        self
    }
}

/// Stores information for a item in a [`NavigationMenu`].
/// A item is either a link or a submenu.
///
/// [`NavigationMenu`]: struct.NavigationMenu.html
#[derive(Debug, Serialize, Clone, PartialEq)]
pub enum NavigationItem {
    Link(Link),
    SubMenu(NavigationMenu),
}

/// Stores information for a link.
///
/// Used in [`NavigationItem`] and [`SidebarItem::Links`].
///
/// [`NavigationItem`]: enum.NavigationItem.html
/// [`SidebarItem::Links`]: enum.SidebarItem.html#Links.v
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Link {
    text: String,
    url: String,
    #[serde(default)]
    attributes: HashMap<String, String>,
}

impl Link {
    pub fn new<T: ToString, U: ToString>(text: T, url: U) -> Link {
        Link {
            text: text.to_string(),
            url: url.to_string(),
            attributes: HashMap::new(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn build_basic_navigation_menu() {
        let mut menu = NavigationMenu::new();
        menu.push_link_to("Home", "/")
            .push_link_to("About", "/about")
            .push_link_to("Blog", "/blog");

        assert_eq!(menu, NavigationMenu { items: vec![
            NavigationItem::Link(Link::new("Home", "/")),
            NavigationItem::Link(Link::new("About", "/about")),
            NavigationItem::Link(Link::new("Blog", "/blog")),
        ] });
    }
}
