use crate::components::profile_badge::ProfileBadge;
use yew::prelude::*;

#[function_component(Profile)]
pub fn profile() -> Html {
    html! {
        <>
            <h1>{"Profile"}</h1>
            <ProfileBadge/>
        </>
    }
}
