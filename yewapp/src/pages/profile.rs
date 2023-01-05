use crate::components::login::Login;
use yew::prelude::*;

#[function_component(Profile)]
pub fn profile() -> Html {
    html! {
        <>
            <h1>{"Profile"}</h1>
            <Login />
        </>
    }
}
