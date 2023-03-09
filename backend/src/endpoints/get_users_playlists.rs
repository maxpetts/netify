use axum::{extract::Query, response::IntoResponse, Json};
use reqwest::StatusCode;
// use rspotify_model::{FullPlaylist, Page};
use rspotify_model::{Followers, Image, PlaylistId, PlaylistItem, PublicUser};
use serde::{Deserialize, Serialize};

// #[derive(Deserialize, Serialize, Debug, Clone)]
// pub struct Details {
//     collaborative: bool,
//     description: String,
//     external_urls: Vec<String>,
//     href: String,
//     id: String,
//     images: Vec<super::get_users_profile::Image>,
//     name: String,
//     // owner:
// }

// #[derive(Deserialize, Serialize, Debug, Clone)]
// pub struct Playlist {
//     items: Details,
// }

#[derive(Deserialize, Serialize, Clone)]
pub struct Request {
    access_token: String,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct PlaylistTracks {
    pub href: String,
    pub total: u32,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct FullPlaylist {
    pub collaborative: bool,
    pub description: Option<String>,
    pub external_urls: std::collections::HashMap<String, String>,
    // pub followers: Followers,
    pub href: String,
    pub id: PlaylistId<'static>,
    pub images: Vec<Image>,
    pub name: String,
    pub owner: PublicUser,
    pub public: Option<bool>,
    pub snapshot_id: String,
    pub tracks: PlaylistTracks,
    pub r#type: String,
    pub uri: String,
}

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

pub async fn get_users_playlists(Query(params): Query<Request>) -> impl IntoResponse {
    println!("get playlists");

    let access_token = params.access_token;

    let url = "https://api.spotify.com/v1/me/playlists";

    let req_builder = reqwest::Client::new()
        .get(url)
        .header("Authorization", &format!("Bearer {}", access_token));

    let res = req_builder
        .send()
        .await
        .expect("error getting playlists request");

    if res.status().is_success() {
        let res_json = res
            .json::<Page<FullPlaylist>>()
            .await
            .expect("error parsing to json");

        return Ok((StatusCode::OK, Json(res_json)));
    } else {
        println!("{:?}", res.text().await);
        return Err(StatusCode::INTERNAL_SERVER_ERROR.into_response());
    }
}
