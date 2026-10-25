pub mod getting_started;
#[cfg(target_arch = "wasm32")]
pub mod i18n_browser;
pub mod i18n_catalog;
#[cfg(not(target_arch = "wasm32"))]
pub mod i18n_server;
pub mod i18n_shared;
pub mod sapid_action;
