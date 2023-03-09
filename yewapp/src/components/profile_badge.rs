use yew::{function_component, html, Html};

use crate::hooks::use_get_profile::Profile;

#[function_component(ProfileBadge)]
pub fn profile_badge() -> Html {
    let _ = crate::hooks::use_validate_logged::use_validate_logged();
    let profile = crate::hooks::use_get_profile::use_get_profile();
    let state = yewdux::prelude::use_store_value::<crate::State>();
    let navigator = yew_router::prelude::use_navigator().unwrap();

    gloo_console::log!(format!("profile {:?}", profile.data));

    if let Some(data) = &profile.data {
        let images = data.images.clone().unwrap();
        html!(
            <>
                {images.iter().map(|image| {
                    html!(<img src={image.clone().url.unwrap()}/>)
                }).collect::<Html>()}
                <p>{format!("Hi {:?}", &data.display_name)}</p>
            </>)
    } else {
        html!("not logged")
    }
}
