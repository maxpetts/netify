use serde::{Deserialize, Serialize};
use spotify_types::playlist::FullPlaylist;
use yew::prelude::*;
use yew_hooks::prelude::*;

/// Paging object
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq, Default)]
pub struct Page<T> {
    pub href: String,
    pub items: Vec<T>,
    pub limit: u32,
    pub next: Option<String>,
    pub offset: u32,
    pub previous: Option<String>,
    pub total: u32,
}

#[hook]
pub fn use_get_playlists() -> UseAsyncHandle<Page<FullPlaylist>, std::string::String> {
    let navigator = yew_router::prelude::use_navigator().unwrap();
    let state = yewdux::prelude::use_store_value::<crate::State>();

    if !state.logged {
        navigator.push(&crate::Route::Login);
    }

    use_async_with_options(
        async move {
            match gloo_net::http::Request::get("http://localhost:3001/getMyPlaylists")
                .query([(
                    "access_token",
                    state.access_token.as_ref().expect("no access_token"),
                )])
                .send()
                .await
            {
                Ok(response) => Ok(response
                    .json::<Page<FullPlaylist>>()
                    .await
                    .expect("Error: parsing token reponse json")),

                Err(err) => {
                    gloo_console::log!(format!("ERR fetch: {:?}", err.to_string()));
                    println!("ERROR");
                    Err(err.to_string())
                }
            }
        },
        UseAsyncOptions::enable_auto(),
    )
}
