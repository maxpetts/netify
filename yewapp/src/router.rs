use crate::pages::{home::Home, profile::Profile};
use gloo::console::log;
use yew::prelude::*;
use yew_router::prelude::*;

#[derive(Clone, Routable, PartialEq)]
pub enum Route {
    #[not_found]
    #[at("/404")]
    NotFound,
    #[at("/profile")]
    Profile,
    #[at("/callback")]
    Callback,
    #[at("/")]
    Home,
}

pub fn switch(routes: &Route) -> Html {
    // let user_context = use_context::<User>().unwrap();

    match routes {
        Route::Home => html! { <Home /> },
        Route::Profile => {
            html! { <Profile/> }
        }
        Route::Callback => {
            html! {"aaa"}
        }
        Route::NotFound => html! {{"404"}},
    }
}
