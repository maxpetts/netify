use gloo_console::log;
use gloo_utils::window;
use yew::prelude::*;
use yewdux::prelude::*;

use crate::{components::atom::button::Button, State};

#[function_component(Login)]
pub fn login() -> Html {
    let (state, dispatch) = use_store::<State>();
    let state_clone = state.clone();
    use_effect_with_deps(move |_| log!(format!("{:?}", state_clone)), state.clone());

    let logged = state.logged;
    // let client_id = store
    //     .state()
    //     .map(|state| state.client_id.clone())
    //     .unwrap_or_default();
    // let hash = store
    //     .state()
    //     .map(|state| state.hash.clone())
    //     .unwrap_or_default();

    let log_inout = Callback::from(move |_| {
        let redirect_uri = "http://localhost:8080/callback";
        let scope = "user-read-private user-read-email";

        let url = format! {
            "https://accounts.spotify.com/authorize?client_id={}&response_type=code&redirect_uri={}&state={}&scope={}",
            state.client_id.clone(),
            redirect_uri,
            state.hash.clone(),
            scope
        };

        let _wind = window().location().assign(url.as_str());

        // swap logged var - should be done in callback..
    });

    html! {
        <Button onclick={log_inout}>{if logged {"Log out"} else {"Log in"}}</Button>
    }
}
