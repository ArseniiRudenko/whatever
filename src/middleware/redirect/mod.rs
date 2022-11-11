/*
this redirect logic is taken from https://github.com/AOx0 with some minor changes.
Go give him a star or something.
*/
pub mod builder;
pub mod scheme;
pub mod service;

pub use self::builder::RedirectSchemeBuilder;
pub use self::scheme::RedirectScheme;