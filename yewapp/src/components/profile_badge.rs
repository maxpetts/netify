use spotify_types::user::Profile;
use yew::{function_component, html, use_node_ref, Callback, Html};
use yew_hooks::{use_event, UseAsyncHandle};
use yew_octicons::{Icon, IconKind};

use crate::components::atom::button::Button;

#[function_component(ProfileBadge)]
pub fn profile_badge() -> Html {
    let _ = crate::hooks::use_validate_logged::use_validate_logged();
    let profile: UseAsyncHandle<Profile, String> = crate::hooks::use_get_profile::use_get_profile();
    let state = yewdux::prelude::use_store_value::<crate::State>();
    let navigator = yew_router::prelude::use_navigator().unwrap();
    let currSong = crate::hooks::use_get_playing_song::use_get_playing_song();

    // let togglePlayButton = {
    //     let navigator = navigator.clone();
    //     let oncliche = Callback::from(async move {
    //         match &toggle_play
    //             .data
    //             .unwrap()
    //             .exec(&state.access_token.unwrap())
    //             .await
    //         {
    //             Ok(resp) => {}
    //             Err(e) => {}
    //         };
    //     });
    //     html! {
    //         <button onclick={oncliche}>{Icon::new(IconKind::Person)}{"Profile"}</button>
    //     }
    // };

    // gloo_console::log!(format!("profile {:?}", profile.data));

    if let Some(data) = &profile.data {
        let images = &data.images;
        html!(
            <>
                {images.iter().map(|image| {
                    html!(<img src={image.url.clone()}/>)
                }).collect::<Html>()}
                <p>{format!("Hi {:?}", &data.display_name)}</p>
                <h3>{if let Some(song_data) = &currSong.data {
                    song_data.item.as_ref().unwrap().name()
                } else {
                    "nothing playing"
                }}</h3>
            </>)
    } else {
        html!("not logged")
    }
}
