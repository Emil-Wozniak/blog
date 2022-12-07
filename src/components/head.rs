use dioxus::prelude::*;

const TAILWIND_CSS: &str = "https://unpkg.com/tailwindcss@^2.2/dist/tailwind.min.css";

pub fn Head(cx: Scope) -> Element {
    cx.render(rsx! {
        head {
              link { rel: "stylesheet", href: "{TAILWIND_CSS}" }
        }
    })
}