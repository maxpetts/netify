use yew::{function_component, html, Html};

use crate::hooks::use_get_profile::Profile;

#[function_component(ProfileBadge)]
pub fn profile_badge() -> Html {
    let _ = crate::hooks::use_validate_logged::use_validate_logged();
    // let profile = crate::hooks::use_get_profile::use_get_profile::<Profile>();
    let profile = use_async
    let state = yewdux::prelude::use_store_value::<crate::State>();
    let navigator = yew_router::prelude::use_navigator().unwrap();

    return html!(format!("Hi {}", profile.display_name));
}
