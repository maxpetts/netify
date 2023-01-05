use std::{
    collections::HashMap,
    thread::{self, spawn},
};

use crate::State;
use base64::encode;
use gloo_console::log;
use gloo_utils::document;
// use reqwest::Response;
use serde::{Deserialize, Serialize};
use timer;
use wasm_bindgen::prelude::*;
use wasm_bindgen::{JsCast, JsValue};
use wasm_bindgen_futures::spawn_local;
use wasm_bindgen_futures::JsFuture;
use web_sys::{Request, RequestInit, RequestMode, Response, Url};
use yew::prelude::*;
use yewdux::prelude::*;
// enum Error {
//     HASH_MISMATCH = "Hash Mismatch",
// }

#[derive(Deserialize, Serialize, Clone, Debug)]
struct ResponseForAccessToken {
    access_token: String,
    token_type: String,
    scope: String,
    expires_in: u64,
    refresh_token: String,
}

#[function_component(Callback)]
pub fn callback() -> Html {
    let (state, dispatch) = use_store::<State>();
    let error: UseStateHandle<Option<&str>> = use_state(|| None);

    if state.as_ref().hash.is_empty() || state.as_ref().auth_token.is_none() {
        let url = Url::new(&document().url().unwrap()).unwrap(); // do some better handling please
        let url_params = web_sys::UrlSearchParams::new_with_str(&url.search());

        if let Ok(UrlSearchParams) = &url_params {
            match (
                url_params.as_ref().unwrap().get("code"),
                url_params.as_ref().unwrap().get("state"),
            ) {
                (Some(code), Some(hash)) => {
                    dispatch.reduce_mut(|state| state.auth_token = Some(code.to_string()));

                    if state.as_ref().hash != hash {
                        error.set(Some("Hash mismatch"));
                    };
                }
                (None, Some(_)) => {
                    let err = url_params.as_ref().unwrap().get("error");
                    match err {
                        Some(e) => match e.as_str() {
                            "access_denied" => error.set(Some(
                                "You denied access. You've seen too much. I cannot let you continue.",
                            )),
                            _ => error.set(Some("un-handled error encountered")),
                        },
                        None => error.set(Some("Spotify hasn't provided an error type..")),
                    }
                }
                (Some(_), None) => error.set(Some("No hash returned by Spotify")),
                (None, None) => error.set(Some("Nothing returned. PANIC PANIC")),
            }
        };
    } else {
        // check whether access token still within valid time frame.

        let response_json = ResponseForAccessToken {
            access_token: String::new(),
            token_type: String::new(),
            scope: String::new(),
            expires_in: 0,
            refresh_token: String::new(),
        };

        match *error {
            Some(err) => {
                log! {err};
                return html! {<p>{err}</p>};
            }
            None => {
                let reponse_json = response_json.clone();
                spawn_local(async move {
                    let mut map = HashMap::new();
                    map.insert("auth_token", state.auth_token.as_ref().unwrap());

                    // log! {"{:?}", Some(state.auth_token)}

                    dispatch.reduce_mut(|state| state.auth_token = None);

                    let req_builder = reqwest::Client::new()
                        .post("http://localhost:3001")
                        .json(&map);

                    let response = req_builder.send().await;

                    if response.is_err() {
                        return;
                    }
                    // log! {"Response text {}", &response.text().await.expect("prop with text in calback")};

                    let response_json = response
                        .unwrap()
                        .json::<ResponseForAccessToken>()
                        .await
                        .expect("error parsing json");

                    log! {"calcing expires in"};
                    let expires_in = chrono::Duration::from_std(std::time::Duration::from_secs(
                        response_json.expires_in,
                    ))
                    .expect("Out of range");

                    log! {"setting states"};
                    dispatch.reduce_mut(|state| {
                        state.access_token = Some(response_json.access_token);
                        state.access_token_recieved =
                            Some(expires_in.to_std().expect("Out of range"));
                        state.access_token_expires =
                            chrono::Utc::now().checked_add_signed(expires_in);
                        state.refresh_token = Some(response_json.refresh_token);
                    })
                });
                return html! {<p>{format!{"{:#?}", response_json}}</p>};
            }
        }
    }

    // log!("{:?}", res);

    html! {<p>{"callback"}</p>}
}

// async fn resolve_promise(promise: js_sys::Promise) -> anyhow::Result<wasm_bindgen::JsValue> {
//     wasm_bindgen_futures::JsFuture::from(promise)
//         .await
//         .map_err(|err| anyhow::anyhow!("{:?}", err))
// }
