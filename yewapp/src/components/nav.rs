use yew::prelude::*;
use yew_router::prelude::*;

use crate::router::Route;

#[function_component(Nav)]
pub fn nav() -> Html {
    let navigator = use_navigator().unwrap();

    let profile_button = {
        let oncliche = Callback::from(move |_| navigator.push(&Route::Profile));

        html! {
            <button onclick={oncliche}>{"Profile"}</button>
        }
    };
    profile_button

    // html! {}
}
