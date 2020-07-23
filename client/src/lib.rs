use wasm_bindgen::prelude::*;

mod agents;
mod components;
mod editor;
mod services;

#[wasm_bindgen(start)]
pub fn run_app() {
    #[cfg(feature = "console_log")]
    console_log::init().expect("failed to initialize logging");

    yew::start_app::<components::BootComponent>();
}
