use dioxus::{prelude::*, };

pub fn page_not_found(cx: Scope) -> Element {
    cx.render(rsx! {
        p { "Oops! The page you are looking for doesn't exist!" }
    })
}