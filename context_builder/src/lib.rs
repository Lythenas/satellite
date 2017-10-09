#![feature(plugin)]
#![plugin(rocket_codegen)]

extern crate rocket;
//extern crate rocket_contrib;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate regex;
extern crate toml;

/// Contains Metadata structs for use in routes.
pub mod meta;

/// Contains helpers for building menus and links.
pub mod nav;

/// Contains [`ContextBuilder`].
/// Which is a useful guard for all routes that return a [`Template`].
///
/// [`TemplateBuilder`]: struct.TemplateBuilder.html
/// [`Template`]: https://api.rocket.rs/rocket_contrib/struct.Template.html
pub mod context_builder;

// TODO rethink how to use this module/library
pub use meta::Metadata;
pub use context_builder::ContextBuilder;
pub use context_builder::TemplateContext;
