
extern crate regex;
//extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate rocket;

mod metadata;

pub use metadata::Metadata;

// TODO
pub struct NavigationBuilder {

}

#[cfg(test)]
mod tests {
    use super::*;

}
