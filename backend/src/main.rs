use std::{string, sync::Arc};

use axum::{
    body,
    extract::{FromRequest, Json, Query, State},
    http::{Method, StatusCode},
    response::{IntoResponse, Response},
    routing::{get, post},
    Router,
};
use axum_macros::debug_handler;
use backend::create_hash;
use serde::{Deserialize, Serialize};
use tower_http::cors::{Any, CorsLayer, Origin};

mod endpoints;

struct AppState {
    client_id: String,
    client_secret: String,
}

#[derive(Deserialize, Debug)]
struct AccessTokenRequest {
    auth_token: String,
}

#[derive(Deserialize, Serialize, Clone)]
struct AccessTokenResponse {
    access_token: String,
    token_type: String,
    scope: String,
    expires_in: u64,
    refresh_token: String,
}

#[derive(Deserialize, Serialize, Clone)]
struct AccessTokenError {
    error: String,
    error_description: String,
}

#[tokio::main]
async fn main() {
    // dotenv::dotenv().expect("Couldn't load .env file");
    dotenv::dotenv().ok();

    let shared_state: Arc<AppState> = Arc::new(AppState {
        client_id: std::env::var("CLIENT_ID").expect("Unable to load client id from env"),
        client_secret: std::env::var("CLIENT_SECRET")
            .expect("Unable to load client secret from env"),
    });

    let app = Router::new()
        .route("/getToken", get(request_access_token))
        .route(
            "/getMyProfile",
            get(endpoints::get_users_profile::get_users_profile),
        )
        .route(
            "/getMyPlaylists",
            get(endpoints::get_users_playlists::get_users_playlists),
        )
        .route(
            "/getPlayingSong",
            get(endpoints::get_playing_song::get_playing_song),
        )
        .layer(
            CorsLayer::new()
                .allow_origin(Any)
                .allow_methods(Any)
                .allow_headers(Any),
        )
        .with_state(shared_state);

    axum::Server::bind(&"0.0.0.0:3001".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn test() -> impl IntoResponse {
    println!("test");
    "response"
}

async fn not_auth() -> impl IntoResponse {
    "not auth"
}

async fn request_user_auth(body: String, State(state): State<AppState>) -> impl IntoResponse {
    let URL: String = format! {
        "https://accounts.spotify.com/authorize?client_id={}&response_type=code&redirect_uri={}&state={}&scope={}",
        state.client_id,
        "http://localhost:8080/callback",
        create_hash(),
        "user-read-private user-read-email"
    };
    let req_builder = reqwest::Client::new().get(URL);
}

async fn request_access_token(Query(params): Query<AccessTokenRequest>) -> impl IntoResponse {
    println!("get token");
    let access_token = if params.auth_token.chars().count() > 0 {
        params.auth_token
    } else {
        return Err(StatusCode::UNPROCESSABLE_ENTITY.into_response()); //is this necessary?
    };
    let client_id = std::env::var("CLIENT_ID").expect("Could not find client_id env var");
    let client_secret =
        std::env::var("CLIENT_SECRET").expect("Could not find client_secret env var");

    let url = "https://accounts.spotify.com/api/token";

    let req_builder = reqwest::Client::new()
        .post(url)
        .form(&[
            ("code", access_token.as_ref()),
            ("redirect_uri", "http://localhost:8080/callback"),
            ("grant_type", "authorization_code"),
        ])
        .header(
            "Authorization",
            format!(
                "Basic {}",
                base64::encode(format! {"{}:{}", client_id, client_secret}),
            ),
        );

    let res = req_builder.send().await.expect("Error getting request");

    if res.status().is_success() {
        let res_json = res
            .json::<AccessTokenResponse>()
            .await
            .expect("Error parsing to JSON");

        // println! {"{:?}", res.text().await};
        // Ok(Json(res_json).into_response())
        return Ok((StatusCode::OK, Json(res_json)));
    } else {
        println!("{:?}", res.text().await);
        return Err(StatusCode::INTERNAL_SERVER_ERROR.into_response());
    }
}
