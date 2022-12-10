use std::{io, vec};

use dioxus::core::to_owned;
use dioxus::prelude::*;
use dioxus::prelude::dioxus_elements::{h4, li, title};
use log::log;
use markdown::to_html;
use serde::{Deserialize, Serialize};
use serde_json::{Result, Value};

use regex::Regex;
use services::supabase::create_client;

use crate::services;
use crate::utils;

#[derive(Serialize, Deserialize, Debug, PartialEq)]
struct Post {
    id: usize,
    user_id: String,
    user_email: String,
    title: String,
    content: String,
    inserted_at: String,
}

//noinspection RsFunctionNaming
pub fn Page(cx: Scope) -> Element {
    cx.render(rsx! {
            div { class: "container p-2 m-2",
                button {
                    onclick: move |_| {
                    },
                    class: "bg-blue-500 hover:bg-blue-700 text-white font-bold py-2 px-4 rounded",
                    "request"
                }
                h1 { "Welcome to My Blog!" }
                Posts()
            }
    })
}

//noinspection RsFunctionNaming
pub fn Posts(cx: Scope) -> Element {
    let future = use_future(&cx, (), |_| async move {
        let client = create_client();
        let result = client.await.from("posts")
            .select("*")
            .execute()
            .await;
        let response = result.unwrap();
        let res = response.text().await;
        let json = res.unwrap();
        let value = json.to_string();
        // log::info!("{}",&value);
        serde_json::from_str::<Vec<Post>>(value.as_str()).unwrap()
    });
    // let json = serde_json::from_str::<Vec<Post>>(text.unwrap().as_str()).unwrap();
    cx.render(match future.value() {
        Some(posts) => rsx! {
            ul {
                class: "py-1 text-sm text-gray-700 dark:text-gray-200",
                posts
                    .iter()
                    .map(|post: &Post| {
                        rsx! {
                            li {
                                class: "block py-1 px-1 hover:bg-gray-100 dark:hover:bg-gray-300 dark:hover:text-white",
                                h4 {
                                    class: "mb-2 text-3xl font-extrabold",
                                    "{post.title}"
                                }
                                Content { content: post.content.clone() }
                            }
                        }
                    })
            }
        },
        None => rsx! {"...loading"}
    })
}

#[derive(Props, PartialEq)]
struct ContentProps {
    content: String,
}

//noinspection RsFunctionNaming
fn Content(cx: Scope<ContentProps>) -> Element {
    let mut props = &cx.props.content.as_str();
    let start_html_tag = Regex::new("&lt;").unwrap();
    let end_html_tag = Regex::new(r"&gt;").unwrap();
    let with_start = start_html_tag.replace_all(props, "<").into_owned();
    let with_end = end_html_tag.replace_all(with_start.as_str(), ">").into_owned();
    let content = to_html(&with_end);
    log::info!("{}",with_end);
    cx.render(rsx! {
            div {
                class: "p-1 m-1 lg:ml-12 text-base bg-white rounded-lg dark:bg-gray-900 shadow-xl",
                dangerous_inner_html: "{content}"
        }
    })
}
