use wasm_bindgen::prelude::*;
use yew::App;

mod app;

#[wasm_bindgen(start)]
pub fn run_app() {
    App::<app::Root>::new().mount_to_body();
}
