use std::fmt::format;
use std::fs;

use chrono;
use dioxus::prelude::*;
use dioxus::prelude::dioxus_elements::div;
use markdown::to_html;

//noinspection RsFunctionNaming
pub fn Posts(cx: Scope) -> Element {
    let md = r#"
    "#;
    let date = format!("{}", chrono::offset::Local::now()).split_whitespace().next().unwrap_or("").to_string();
    let contents = to_html(md);
    // let contents = include_str!(html);
    cx.render(rsx! {
            div { class: "flex justify-between px-4 mx-auto max-w-screen-xl ",
                article { class: "mx-auto w-full max-w-2xl format format-sm sm:format-base lg:format-lg format-blue dark:format-invert",
                    header { class: "mb-4 lg:mb-6 not-format",
                        address { class: "flex items-center mb-6 not-italic",
                            div { class: "inline-flex items-center mr-3 text-sm text-gray-900 dark:text-white",
                                img { class: "mr-4 w-16 h-16 rounded-full",
                                    src: "/me.jpg" ,
                                    alt: "my image"
                                }
                                div {
                                    a { href: "#", rel: "author",
                                        class: "text-xl font-bold text-gray-900 dark:text-white",
                                        "Emil Woźniak"
                                    }
                                    p { class: "text-base font-light text-gray-500 dark:text-gray-400",
                                        "Fullstack web developer",
                                    }
                                    p { class: "text-base font-light text-gray-500 dark:text-gray-400",
                                        "{date}"
                                    }
                                }
                            }
                        }
                    h1 { class: "mb-4 text-3xl font-extrabold leading-tight text-gray-900 lg:mb-6 lg:text-4xl dark:text-white",
                        "Best practices for successful prototypes"
                    }
                    div {
                        class: "markdown",
                        dangerous_inner_html: "{contents}",
                    }
                }
            }
            aside { class: "py-8 lg:py-24 bg-gray-50 dark:bg-gray-800",
                div { class: "px-4 mx-auto max-w-screen-xl",
                    h2 { class: "mb-8 text-2xl font-bold text-gray-900 dark:text-white",
                        "Related articles"
                        div { class: "grid gap-12 sm:grid-cols-2 lg:grid-cols-4",
                            article { class: "max-w-md",
                                a { href: "#",
                                    img {
                                        src: "https://flowbite.s3.amazonaws.com/blocks/marketing-ui/article/blog-1.png",
                                        class: "mb-5 rounded-lg",
                                        alt: "Image 1",
                                    }
                                }
                                h2 { class: "mb-2 text-md font-bold leading-tight text-gray-900 dark:text-white",
                                    a { href: "#", "Our first office" }
                                }
                                p { class: "mb-4 font-light text-gray-500 dark:text-gray-400",
                                    "Over the past year, Volosoft has undergone many changes! After months of preparation.",
                                }
                                a { href: "#", class: "inline-flex items-center font-medium underline underline-offset-4 text-primary-600 dark:text-primary-500 hover:no-underline",
                                    "Read in 2 minutes"
                                }
                            }
                        }
                    }
                }
            }
        }
    })
}