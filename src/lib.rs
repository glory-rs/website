pub mod app;
pub mod components;
pub mod examples;
pub mod i18n;
pub mod metadata;
pub mod pages;
pub mod routes;

#[cfg(target_arch = "wasm32")]
pub mod web;

pub use app::{AppState, app};
pub use i18n::Locale;
pub use routes::{Route, SiteRoute};
