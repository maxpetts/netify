use yew::prelude::*;

use crate::components::organisms::playlist::Playlist;

#[function_component(Home)]
pub fn home() -> Html {
    let playlists = crate::hooks::use_get_playlists::use_get_playlists();
    html! {
        <>
            <h1>{"Home"}</h1>
            {if let Some(data) = &playlists.data {
                let playlists = data.clone();
                html!(
                    <>
                        {playlists.items.iter().map(|playlist| {
                            html!(<Playlist playlist={playlist.to_owned()}/>)
                        }).collect::<Html>()}
                    </>)
            } else {
                html!(<b>{"Mo playlists"}</b>)
            }}
        </>
    }
}
