use axum::{extract::Query, response::IntoResponse, Json};
use reqwest::StatusCode;
use serde::{Deserialize, Serialize};
use spotify_types::curr_playing::CurrentlyPlaying;

#[derive(Deserialize, Serialize, Clone)]
pub struct Request {
    access_token: String,
}

pub async fn get_playing_song(Query(params): Query<Request>) -> impl IntoResponse {
    println!("get curr song");

    let access_token = params.access_token;

    let url = "https://api.spotify.com/v1/me/player/currently-playing";

    let req_builder = reqwest::Client::new()
        .get(url)
        .header("Authorization", &format!("Bearer {}", access_token));

    let res = req_builder
        .send()
        .await
        .expect("error getting current playing request");

    if res.status().is_success() {
        let res_json = res
            .json::<CurrentlyPlaying>()
            .await
            .expect("error parsing current song to json");

        // println!("{:#?}", res.text().await);
        // let res_json = "";

        return Ok((StatusCode::OK, Json(res_json)));
    } else {
        println!("{:?}", res.text().await);
        return Err(StatusCode::INTERNAL_SERVER_ERROR.into_response());
    }
}
