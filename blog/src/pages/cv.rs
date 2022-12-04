use dioxus::{prelude::*, };

pub fn cv(cx: Scope) -> Element {
    cx.render(rsx!(
        body {
            div {
                h1 { "CV" }
            }
        }
    ))
}
