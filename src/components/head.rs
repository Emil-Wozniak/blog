use dioxus::prelude::*;
use dioxus_helmet::Helmet;

const TAILWIND_CSS: &str = "https://unpkg.com/tailwindcss@^2.2/dist/tailwind.min.css";

//noinspection RsFunctionNaming
#[inline_props]
pub fn Head(cx: Scope) -> Element {
    cx.render(rsx! {
         Helmet {
              title { "EjDEV | Main" }
              link { rel: "stylesheet", href: "{TAILWIND_CSS}" }
              style {
                [r#"
                .language-sql {
                  font-family: monospace;
                  background-color: black;
                  color: white;
                }
                "#
                ]
              }
        }
    })
}