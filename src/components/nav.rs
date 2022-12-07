use dioxus::{
    prelude::*,
    router::{Link},
};

pub fn Navbar(cx: Scope) -> Element {
    cx.render(rsx! {
        section { class: "w-full top-0 overflow-hidden mb-6 py-6 px-6 border-r bg-sky-500 hover:bg-sky-700",
            nav { class: "block h-full ",
                div { class: "float-left text-3xl font-bold font-heading",
                    a { class: "m-8", Link { to: "/", "Home"} }
                    a { class: "m-8", Link { to: "/blog", "Blog"} }
                    a { class: "m-8", Link { to: "/cv", "CV"} }
                    a { class: "m-8", Link { to: "/secret", "Secret"} }
                }
            }
        }
    })
}