use dioxus::{
    prelude::*,
};

pub fn homepage(cx: Scope) -> Element {
    cx.render(rsx! {
            div {
                h1 { "Welcome to My Blog!" }
            }
    })
}