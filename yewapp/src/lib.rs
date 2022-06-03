mod components;
mod pages;
mod router;

use gloo::console::log;
use rand::{distributions::Alphanumeric, Rng};
use router::{switch, Route};
use yew::prelude::*;
use yew_router::prelude::*;
use yewdux::prelude::*;
use yewdux_functional::use_store;

use serde::{Deserialize, Serialize};

#[derive(Default, Clone, Deserialize, Serialize)]
struct State {
    hash: Option<String>,
}

impl Persistent for State {
    fn area() -> Area {
        Area::Session
    }
}

#[function_component(App)]
pub fn app() -> Html {
    let store = use_store::<PersistentStore<State>>();
    let hash = store
        .state()
        .map(|state| state.hash.as_ref())
        .unwrap_or_default();

    if hash.is_none() {
        store.dispatch().reduce(|state| {
            log! {"Setting hash"};
            state.hash = Some(
                rand::thread_rng()
                    .sample_iter(&Alphanumeric)
                    .take(16)
                    .map(char::from)
                    .collect::<String>(),
            )
        });
    };

    html! {
        <BrowserRouter>
            <Switch<Route> render={Switch::render(switch)} />
            <p>{format!{"{:#?}", hash}}</p>
        </BrowserRouter>
    }
}
