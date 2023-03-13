use yew::prelude::*;
use yew_router::prelude::*;

use crate::components::atom::button::Button;
use crate::{components::login::Login, router::Route};

#[function_component(Nav)]
pub fn nav() -> Html {
    let container = stylist::style!(
        r#"
        background-color: #dfd;
        "#
    )
    .expect("err in container style");

    let navigator = use_navigator().unwrap();

    let oncliche = Callback::from(move |_| navigator.push(&Route::Profile));
    html!(
        <div class={container}>
            <Button onclick={oncliche}>{"Profile"}</Button>
            <Login />
        </div>
    )
}
