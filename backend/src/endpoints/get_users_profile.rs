use axum::{extract::Query, response::IntoResponse, Json};
use reqwest::StatusCode;
use serde::{Deserialize, Serialize};
use spotify_types::user::Profile;

// #[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
// pub struct ExplicitContent {
//     pub filter_enabled: Option<bool>,
//     pub filter_locked: Option<bool>,
// }
// #[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
// pub struct Followers {
//     pub href: Option<String>,
//     pub total: Option<u32>,
// }

// #[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
// pub struct Image {
//     pub url: Option<String>,
//     pub height: Option<u32>,
//     pub width: Option<u32>,
// }

// #[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
// pub struct ExternalUrls {
//     pub spotify: Option<String>,
// }

// #[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
// pub struct Profile {
//     pub country: Option<String>,
//     pub display_name: Option<String>,
//     pub email: Option<String>,
//     pub explicit_content: Option<ExplicitContent>,
//     pub external_urls: Option<ExternalUrls>,
//     pub followers: Option<Followers>,
//     pub href: Option<String>,
//     pub id: Option<String>,
//     pub images: Option<Vec<Image>>,
//     pub product: Option<String>,
//     pub r#type: Option<String>,
//     pub uri: Option<String>,
// }

#[derive(Deserialize, Serialize, Clone)]
pub struct Request {
    access_token: String,
}

pub async fn get_users_profile(Query(params): Query<Request>) -> impl IntoResponse {
    println!("get profile");
    let access_token = params.access_token;

    let url = "https://api.spotify.com/v1/me";

    let req_builder = reqwest::Client::new()
        .get(url)
        .header("Authorization", &format!("Bearer {}", access_token));

    let res = req_builder
        .send()
        .await
        .expect("error getting profile request");

    if res.status().is_success() {
        let res_json = res.json::<Profile>().await.expect("error parsing to json");
        // println!("{:?}", res_json);

        return Ok((StatusCode::OK, Json(res_json)));
    } else {
        println!("{:?}", res.text().await);
        return Err(StatusCode::INTERNAL_SERVER_ERROR.into_response());
    }
}
