use std::io;

use dioxus::prelude::*;
use dioxus::prelude::dioxus_elements::i;
use log::log;
use serde::{Deserialize, Serialize};
use serde_json::{Result, Value};
use services::supabase::create_client;
use crate::services;
use crate::utils;

#[derive(Serialize, Deserialize, Debug)]
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
    let future = use_future(&cx, (), |_| async move {
        let client = create_client();
        let result = client.await.from("posts")
            .select("*")
            .execute()
            .await;
        if result.is_ok() {
            let text = result.unwrap().text().await;
            let json = serde_json::from_str::<Vec<Post>>(text.unwrap().as_str()).unwrap();
            log::info!("{:?}", &json);
            return Ok(json)
        } else {
            log::error!("{:?}", result.unwrap_err());
            return None
        }
    });

    cx.render(rsx! {
            div { class: "container p-2 m-2",
                button {
                    onclick: move |_| {
                        // get_data();
                    },
                    class: "bg-blue-500 hover:bg-blue-700 text-white font-bold py-2 px-4 rounded",
                    "request"
                }
                h1 { "Welcome to My Blog!" }

            }
    })
}
