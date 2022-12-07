use dioxus::prelude::*;

pub fn Page(cx: Scope) -> Element {
    cx.render(rsx! {
        p { "This page is not to be viewed!" }
        Redirect { to: "/" } // NEW
    })
}