use rand::{distributions::Alphanumeric, Rng};
use yew::prelude::*; // 0.8

enum Msg {
    LogIn,
    LogOut,
}

struct UserComponent {
    logged: bool,
}

impl Component for UserComponent {
    type Message = Msg;
    type Properties = ();

    fn create(_: &yew::Context<Self>) -> Self {
        Self { logged: false }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::LogIn => {
                println!("Logging in");
                self.logged = true;

                let client_id = "dcd5f7be4a1f450a8c23297b83a09cd3";
                let client_secret = "d3d5eb4d4c9a4615b44d107c9cb83918";
                let redirect_uri = "http://localhost:8080/callback";
                let state: String = rand::thread_rng()
                    .sample_iter(&Alphanumeric)
                    .take(16)
                    .map(char::from)
                    .collect();
                let scope = "user-read-private user-read-email";

                let _window = web_sys::window()
                    .unwrap()
                    .location()
                    .assign(format!{
                        "https://accounts.spotify.com/authorize?client_id={}&response_type=code&redirect_uri={}&state={}&scope={}",
                        client_id,
                        redirect_uri,
                        state,
                        scope
                    }.as_str()); // would window ever be unavailable?

                true
            }
            Msg::LogOut => {
                println!("Logging out");
                self.logged = false;
                true
            }
        }
    }

    fn view(&self, ctx: &yew::Context<Self>) -> Html {
        let link = ctx.link();

        html! {
            <>
                {self.logged}
                if self.logged {
                    <button onclick={link.callback(|_| Msg::LogOut)}>{"Log Out"}</button>

                } else {
                    <button onclick={link.callback(|_| Msg::LogIn)}>{"Log in"}</button>
                }
            </>
        }
    }
}

fn main() {
    println!("Starting");
    yew::start_app::<UserComponent>();
}
