use gloo::console::log;
use std::collections::HashMap;
use yew::prelude::*;
use yew_router::prelude::*;

use crate::App;

#[function_component(Callback)]
pub fn callback() -> Html {
    // let user_context = use_context::<User>().unwrap().clone();

    let url_queries = use_location()
        .unwrap()
        .query::<HashMap<String, String>>()
        .unwrap();

    let code = url_queries.get("code").unwrap(); // TODO: handle unwrap better
    let state = url_queries.get("state").unwrap();

    let app_state = App::app_state.clone();

    // log!(
    //     *state == user_context.state,
    //     "url state: ",
    //     state,
    //     "curated state: ",
    //     user_context.state
    // );

    // check state

    html! {
        <h1>{"callback"}</h1>
    }
}
