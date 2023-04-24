mod components;
mod pages;

use leptos::*;
use leptos_router::*;

use crate::pages::{
    home::{Home, HomeProps},
    profile::{Profile, ProfileProps},
};

pub fn main() {
    mount_to_body(|cx| {
        view! {cx,
            <Router>
                <nav>
                    <A href="">"Home"</A>
                    <A href="me">"Profile"</A>
                </nav>
                <main>
                    <Routes>
                        <Route path="/" view=|cx| view!{cx, <Home/>} />
                        <Route path="/me" view=|cx| view!{cx, <Profile/>} />
                    </Routes>
                </main>
            </Router>
        }
    })
}
