use std::io;
use std::io::{Read, Write};
use dioxus::{
    prelude::*,
    router::{Link, Redirect, Route, Router, use_route}, // UPDATED
};

pub mod pages;
pub mod components;
pub mod router;
pub mod utils;
pub mod services;
use router::routes;

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    console_error_panic_hook::set_once();
    dioxus::web::launch(app);
}

fn app(cx: Scope) -> Element {
    routes(cx)
}

