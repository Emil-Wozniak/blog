use dioxus::{
    prelude::*,
    router::{Link, Redirect, Route, Router, use_route},
};

use crate::components::nav::Navbar;
use crate::components::head::Head;
use crate::pages::{home,blog,cv,not_found, secret_page};

pub fn routes(cx: Scope) -> Element {
     cx.render(rsx! {
     Head()
     body {
            Router {
                Navbar()
                Route { to: "/", home::Page()}
                Route { to: "/cv", cv::CV() }
                Route { to: "/blog", blog::Posts() }
                Route { to: "/secret", secret_page::Page() }
                Route { to: "", not_found::page_not_found()}
            }
        }
    })
}