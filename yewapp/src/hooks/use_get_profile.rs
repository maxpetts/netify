// use rspotify_model::PublicUser;
use spotify_types::user::Profile;
use yew::prelude::*;
use yew_hooks::prelude::*;

#[hook]
pub fn use_get_profile() -> UseAsyncHandle<Profile, std::string::String> {
    let navigator = yew_router::prelude::use_navigator().unwrap();
    let state = yewdux::prelude::use_store_value::<crate::State>();

    if !state.logged {
        navigator.push(&crate::Route::Login);
    }

    use_async_with_options(
        async move {
            match gloo_net::http::Request::get("http://localhost:3001/getMyProfile")
                .query([("access_token", state.access_token.as_ref().unwrap())])
                .send()
                .await
            {
                Ok(response) => Ok(response
                    .json::<Profile>()
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
