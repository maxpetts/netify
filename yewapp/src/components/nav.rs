use yew::prelude::*;
use yew_octicons::Icon;
use yew_octicons::IconKind;
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

    let homeButton = {
        let navigator = navigator.clone();
        let oncliche = Callback::from(move |_| navigator.push(&Route::Home));
        html! {
            <Button onclick={oncliche}>{Icon::new(IconKind::Home)}{"Home"}</Button>
        }
    };

    let profileButton = {
        let navigator = navigator.clone();
        let oncliche = Callback::from(move |_| navigator.push(&Route::Profile));
        html! {
            <Button onclick={oncliche}>{Icon::new(IconKind::Person)}{"Profile"}</Button>
        }
    };

    html!(
        <div class={container}>
            {homeButton}
            {profileButton}
            <Login />
        </div>
    )
}
