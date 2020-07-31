use wasm_bindgen::prelude::*;

mod agents;
mod boot;
mod components;
mod routes;
mod services;
mod utils;

#[wasm_bindgen(start)]
pub fn run_app() {
    #[cfg(feature = "console_log")]
    console_log::init().expect("failed to initialize logging");

    yew::start_app::<boot::BootComponent>();
}
