use std::string;

use axum::{
    body,
    extract::FromRequest,
    http::{Method, StatusCode},
    response::{IntoResponse, Response},
    routing::{get, post},
    Json, Router,
};
use axum_macros::debug_handler;
use serde::{Deserialize, Serialize};
use tower_http::cors::{Any, CorsLayer, Origin};

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
    dotenv::dotenv().expect("Couldn't load .env file");
    let app = Router::new().route("/", post(request_access_token)).layer(
        CorsLayer::new()
            .allow_origin(Any)
            .allow_methods(Any)
            .allow_headers(Any),
    );

    axum::Server::bind(&"0.0.0.0:3001".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn not_auth() -> impl IntoResponse {
    "not auth"
}

async fn request_user_auth() -> impl IntoResponse {
    "Get user auth"
}
#[debug_handler]
async fn request_access_token(Json(payload): Json<AccessTokenRequest>) -> impl IntoResponse {
    let access_token = if payload.auth_token.chars().count() > 0 {
        payload.auth_token
    } else {
        return Err(StatusCode::UNPROCESSABLE_ENTITY.into_response());
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
                base64::encode(format! {"{}:{}", client_id,client_secret}),
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
    }

    Err((
        StatusCode::INTERNAL_SERVER_ERROR,
        // res.json::<AccessTokenError>()
        res.text().await.expect("err parsing the err!!"),
    ))
    .into_response();
}

#[derive(Deserialize, Debug)]
struct AccessTokenRequest {
    auth_token: String,
}
