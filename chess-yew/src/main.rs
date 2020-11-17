#![recursion_limit = "1024"]

pub mod agent;
pub mod app;
pub mod components;
pub mod types;

use app::App;
use yew::prelude::*;

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    yew::start_app::<App>();
}
