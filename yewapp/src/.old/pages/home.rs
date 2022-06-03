use yew::prelude::*;
use yew_router::prelude::*;

use crate::router::Route;

#[function_component(Home)]
pub fn home() -> Html {
    let hist = use_history().unwrap();
    let onclick = Callback::once(move |_| hist.push(Route::Profile));

    html! {
        <>
            <h1>{"Home"}</h1>
            <button {onclick}>{"Login"}</button>
        </>
    }
}
