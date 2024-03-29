use axum::{extract::Query, response::IntoResponse, Json};
use reqwest::StatusCode;
use serde::{Deserialize, Serialize};
use spotify_types::{page::Page, playlist::FullPlaylist};

#[derive(Deserialize, Serialize, Clone)]
pub struct Request {
    access_token: String,
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
