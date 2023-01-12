use yew::prelude::*;
use yew_router::prelude::*;

use crate::{components::login::Login, router::Route};

#[function_component(Nav)]
pub fn nav() -> Html {
    let navigator = use_navigator().unwrap();

    let oncliche = Callback::from(move |_| navigator.push(&Route::Profile));
    html!(
        <>
            <button onclick={oncliche}>{"Profile"}</button>
            <Login />
        </>
    )
}
