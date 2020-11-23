#![recursion_limit = "1024"]

pub mod agent;
pub mod app;
pub mod components;
pub mod types;

use app::App;
use yew::prelude::*;

use console_error_panic_hook;
use std::panic;

fn main() {
    panic::set_hook(Box::new(console_error_panic_hook::hook));
    wasm_logger::init(wasm_logger::Config::default());
    yew::start_app::<App>();
}
