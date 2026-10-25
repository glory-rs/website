pub mod app;
pub mod components;
pub mod pages;
pub mod routes;

#[cfg(target_arch = "wasm32")]
pub mod web;

pub use app::{AppState, app};
pub use routes::Route;
