pub mod app;
pub mod components;
pub mod error_template;
pub mod errors;
pub mod functions;
pub mod layouts;
pub mod models;
pub mod providers;
pub mod routes;
#[cfg(feature = "ssr")]
pub mod state;

#[cfg(feature = "hydrate")]
#[wasm_bindgen::prelude::wasm_bindgen]
pub fn hydrate() {
    use crate::app::*;
    _ = console_log::init_with_level(log::Level::Debug);
    console_error_panic_hook::set_once();
    leptos::mount::hydrate_body(App);
}
