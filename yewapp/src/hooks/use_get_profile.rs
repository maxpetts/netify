use yew::prelude::*;
use yew_hooks::prelude::*;

#[derive(serde::Serialize, serde::Deserialize, Debug)]
struct ExplicitContent {
    filter_enabled: Option<bool>,
    filter_locked: Option<bool>,
}
#[derive(serde::Serialize, serde::Deserialize, Debug)]
struct Followers {
    href: Option<String>,
    total: Option<u32>,
}

#[derive(serde::Serialize, serde::Deserialize, Debug)]
struct Image {
    url: Option<String>,
    height: Option<u32>,
    width: Option<u32>,
}

#[derive(serde::Serialize, serde::Deserialize, Debug)]
struct ExternalUrls {
    spotify: Option<String>,
}

#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct Profile {
    country: Option<String>,
    display_name: Option<String>,
    email: Option<String>,
    explicit_content: Option<ExplicitContent>,
    external_urls: Option<ExternalUrls>,
    followers: Option<Followers>,
    href: Option<String>,
    id: Option<String>,
    images: Option<Vec<Image>>,
    product: Option<String>,
    r#type: Option<String>,
    uri: Option<String>,
}

#[hook]
pub fn use_get_profile() -> Profile {
    let navigator = yew_router::prelude::use_navigator().unwrap();
    let state = yewdux::prelude::use_store_value::<crate::State>();
    let profile: UseStateHandle<Profile> = yew::use_state(|| Profile {
        country: None,
        display_name: None,
        email: None,
        explicit_content: None,
        external_urls: None,
        followers: None,
        href: None,
        id: None,
        images: None,
        product: None,
        r#type: None,
        uri: None,
    });

    use_effect_with_deps(
        move |_| use_async(async move { Ok(String::from("string")) }),
        navigator.clone(),
    );

    // use_effect(move || {
    //     gloo_console::log!(format!("{:?}", state));
    if !state.logged {
        navigator.push(&crate::Route::Login);
        // return;
    }

    //     yew::platform::spawn_local(async move {
    //         // let user_profile: Profile;
    //         let response = gloo_net::http::Request::get("https://api.spotify.com/v1/me")
    //             .header(
    //                 "Authorization",
    //                 &format!("Bearer {}", state.access_token.as_ref().unwrap()),
    //             )
    //             .send()
    //             .await;

    //         match response {
    //             Ok(response) => {
    //                 gloo_console::log!("{:?}", response.as_raw());
    //                 let user_profile = response.json::<Profile>().await;
    //                 match user_profile {
    //                     Ok(user_profile) => {
    //                         gloo_console::log!(format!("{:?}", user_profile));
    //                         return user_profile;
    //                     }
    //                     Err(err) => gloo_console::log!(err.to_string()),
    //                 }
    //             }
    //             Err(err) => (),
    //         }
    //     });
    // });
    *profile
}
