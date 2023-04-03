use spotify_types::user::Profile;
use yew::{function_component, html, Html};
use yew_hooks::UseAsyncHandle;

#[function_component(ProfileBadge)]
pub fn profile_badge() -> Html {
    let _ = crate::hooks::use_validate_logged::use_validate_logged();
    let profile: UseAsyncHandle<Profile, String> = crate::hooks::use_get_profile::use_get_profile();
    let state = yewdux::prelude::use_store_value::<crate::State>();
    let navigator = yew_router::prelude::use_navigator().unwrap();
    let currSong = crate::hooks::use_get_playing_song::use_get_playing_song();

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
