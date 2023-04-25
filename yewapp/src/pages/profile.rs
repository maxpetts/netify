use crate::components::{organisms::playlist::Playlist, profile_badge::ProfileBadge};
use yew::prelude::*;

#[function_component(Profile)]
pub fn profile() -> Html {
    let playlists = crate::hooks::use_get_playlists::use_get_playlists();

    html! {
            <>
            <h1>{"Profile"}</h1>
            <ProfileBadge/>

            // {if let Some(data) = &playlists.data {
            //     playlists
            //     .data
            //     .as_ref()
            //     .expect("no playlists")
            //     .iter()
            //     .map(|playlist| html!(<h3>{playlist.name.clone()}</h3>))
            //     .collect::<Html>()
            // }}

        {
            if let Some(data) = &playlists.data {
            let playlists = data.clone();
            html!(
                <>
                    {playlists.items.iter().map(|playlist| {
                        // html!({playlist.name.clone()})
                        html!(<Playlist playlist={playlist.to_owned()}/>)
                    }).collect::<Html>()}
                </>)
            } else {
                html!(<b>{"Mo playlists"}</b>)
            }
        }
        </>
    }
}
