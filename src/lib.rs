pub mod app;
pub mod assets;
pub mod components;
pub mod examples;
pub mod i18n;
pub mod metadata;
pub mod navigation;
pub mod pages;
pub mod routes;
pub mod routing;

#[cfg(target_arch = "wasm32")]
pub mod web;

pub use app::app;
pub use i18n::Locale;
pub use routes::{Route, SiteRoute};
