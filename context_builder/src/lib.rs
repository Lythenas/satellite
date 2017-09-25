#![feature(test)]
#[cfg(test)]
extern crate test;

extern crate regex;
//extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate rocket;

pub mod metadata;
pub mod navigation;

pub use metadata::Metadata;
pub use navigation::NavigationMenu;

#[cfg(test)]
mod tests {
    //use super::*;

}
