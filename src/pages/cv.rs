use dioxus::prelude::*;

pub fn CV(cx: Scope) -> Element {
    cx.render(rsx!(
        body {
            div {
                h1 { "CV" }
            }
        }
    ))
}
