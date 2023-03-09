// use rspotify_model::PublicUser;
use yew::prelude::*;
use yew_hooks::prelude::*;

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
pub struct ExplicitContent {
    pub filter_enabled: Option<bool>,
    pub filter_locked: Option<bool>,
}
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
pub struct Followers {
    pub href: Option<String>,
    pub total: Option<u32>,
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
pub struct Image {
    pub url: Option<String>,
    pub height: Option<u32>,
    pub width: Option<u32>,
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
pub struct ExternalUrls {
    pub spotify: Option<String>,
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
pub struct Profile {
    pub country: Option<String>,
    pub display_name: Option<String>,
    pub email: Option<String>,
    pub explicit_content: Option<ExplicitContent>,
    pub external_urls: Option<ExternalUrls>,
    pub followers: Option<Followers>,
    pub href: Option<String>,
    pub id: Option<String>,
    pub images: Option<Vec<Image>>,
    pub product: Option<String>,
    pub r#type: Option<String>,
    pub uri: Option<String>,
}

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