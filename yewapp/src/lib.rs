mod components;
mod pages;
mod router;

use std::{borrow::BorrowMut, process::exit, time::Duration};

use chrono::{DateTime, Utc};
use gloo_console::log;
use rand::{distributions::Alphanumeric, Rng};
use router::{switch, Route};
use web_sys::console::log;
use yew::{functional::*, prelude::*};
use yew_router::prelude::*;
use yewdux::prelude::*;

use crate::components::nav::Nav;
use serde::{Deserialize, Serialize};

#[derive(Default, Clone, PartialEq, Eq, Deserialize, Serialize, Store, Debug)]
pub struct State {
    hash: String,
    client_id: String,
    logged: bool,
    auth_token: Option<String>,
    access_token: Option<String>,
    access_token_recieved: Option<DateTime<Utc>>,
    access_token_expires: Option<DateTime<Utc>>,
    refresh_token: Option<String>,
}

pub fn serialize_dt<S>(dt: &Option<DateTime<Utc>>, serializer: S) -> Result<S::Ok, S::Error>
where
    S: serde::Serializer,
{
    match dt {
        Some(dt) => chrono::serde::ts_seconds::serialize(dt, serializer),
        None => serializer.serialize_none(),
        _ => unreachable!(),
    }
}

pub fn create_hash() -> String {
    log! {"Creating hash"};

    rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(16)
        .map(char::from)
        .collect::<String>()
}

#[function_component(App)]
pub fn app() -> Html {
    let dispatch = Dispatch::<State>::new();

    dispatch.reduce_mut(|state| {
        state.client_id = "dcd5f7be4a1f450a8c23297b83a09cd3".to_string();
        if state.hash.is_empty() {
            state.hash = create_hash();
        };
    });

    html! {
        <BrowserRouter>
            <Nav/>
            <Switch<Route> render={switch} />
        </BrowserRouter>
    }
}
