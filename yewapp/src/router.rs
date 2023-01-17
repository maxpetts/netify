use crate::{
    components::login::Login,
    pages::{callback::Callback, home::Home, profile::Profile},
};use yew::prelude::*;
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
    #[at("/login")]
    Login,
    #[at("/")]
    Home,
}

pub fn switch(routes: Route) -> Html {
    match routes {
        Route::Home => html! { <Home /> },
        Route::Profile => {
            html! { <Profile/> }
        }
        Route::Callback => {
            html! { <Callback /> }
        }
        Route::Login => {
            html! {<Login/> }
        }
        Route::NotFound => html! {{"404"}},
    }
}
