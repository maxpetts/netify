use gloo::console::log;
use rand::{distributions::Alphanumeric, Rng};
use std::collections::HashMap;
use yew::prelude::*;
use yew_router::prelude::*;

use crate::State;

pub enum Msg {
    LogIn,
    LogOut,
}
#[derive(Clone, PartialEq)]
pub struct UserComponent {
    // logged: bool,
    error: Option<String>,
    // state: String,
    // auth_token: Option<String>,
    // refr_token: Option<String>,
}

// #[derive(Properties, PartialEq)]
// pub struct UserProps {
//     pub state: String,
// }

// struct for auth?

impl Component for UserComponent {
    type Message = Msg;
    type Properties = ();

    fn create(ctx: &yew::Context<Self>) -> Self {
        //     let (user, _) = ctx
        //         .link()
        //         .context::<User>(Callback::noop())
        //         .expect("ctx to be set");
        let user = ctx
            .link()
            .context::<State>(Callback::noop())
            .unwrap()
            .0
            .clone();
        Self { error: None }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        let mut state = ctx //cant be mut - usestate callbacks etc
            .link()
            .context::<State>(Callback::noop())
            .unwrap()
            .0
            .clone();

        match msg {
            Msg::LogIn => {
                println!("Logging in");
                state.logged = true;

                log!("iserstate: ", &state.state.clone().unwrap());

                let client_id = "dcd5f7be4a1f450a8c23297b83a09cd3";
                let client_secret = "wowoow";
                let redirect_uri = "http://localhost:8080/callback";
                let scope = "user-read-private user-read-email";

                let _window = web_sys::window()
                    .unwrap()
                    .location()
                    .assign(format!{
                        "https://accounts.spotify.com/authorize?client_id={}&response_type=code&redirect_uri={}&state={}&scope={}",
                        client_id,
                        redirect_uri,
                        &state.state.unwrap(),
                        scope
                    }.as_str()); // would window ever be unavailable?

                true
            }
            Msg::LogOut => {
                println!("Logging out");
                state.logged = false;
                true
            }
        }
    }

    fn view(&self, ctx: &yew::Context<Self>) -> Html {
        let link = ctx.link();

        let user = link.context::<State>(Callback::noop()).unwrap().0.clone();

        html! {
            <>
                {user.logged}
                if self.error == None {
                if user.logged {
                    <button onclick={link.callback(|_| Msg::LogOut)}>{"Log Out"}</button>

                } else {
                    <button onclick={link.callback(|_| Msg::LogIn)}>{"Log in"}</button>
                }
            } else {
                {format!("Error validating: {:?}", self.error)}
            }
            </>
        }
    }
}
