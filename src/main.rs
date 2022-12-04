use dioxus::{
    prelude::*,
    router::{Link, Redirect, Route, Router, use_route}, // UPDATED
};

mod components {
    pub mod nav;
}
mod pages {
    pub mod home;
    pub mod blog;
    pub mod cv;
    pub mod not_found;
}

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    console_error_panic_hook::set_once();
    dioxus::web::launch(app);
}

fn app(cx: Scope) -> Element {
    cx.render(rsx! {
        body {
            Router {
            head { link { rel: "stylesheet", href: "https://unpkg.com/tailwindcss@^2.2/dist/tailwind.min.css" } }
                components::nav::navbar()
                Route { to: "/", pages::home::homepage()}
                Route { to: "/cv", pages::cv::cv() }
                Route { to: "/blog", pages::blog::posts() }
                Route { to: "/secret", self::secret_page {}}
                Route { to: "", pages::not_found::page_not_found()}
            }
        }
    })
}

fn secret_page(cx: Scope) -> Element {
    cx.render(rsx! {
        p { "This page is not to be viewed!" }
        Redirect { to: "/" } // NEW
    })
}


