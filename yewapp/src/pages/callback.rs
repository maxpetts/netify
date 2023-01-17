use std::rc::Rc;

use crate::State;
use gloo_console::log;
use gloo_utils::document;
use web_sys::Url;
use yew::{function_component, html, Html};
use yew_router::prelude::use_navigator;
use yewdux::prelude::use_store;

#[derive(serde::Serialize, serde::Deserialize)]
struct TokenResponse {
    access_token: String,
    // token_type: String, // SHould be enum, Moved to backend
    expires_in: u64,
    // refresh_token: String, Moved to backend
}

#[function_component(Callback)]
pub fn callback() -> Html {
    let (state, dispatch) = use_store::<State>();
    let navigator = use_navigator().unwrap();

    let url = Url::new(&document().url().unwrap()).unwrap(); // do some better handling please
    let url_params = web_sys::UrlSearchParams::new_with_str(&url.search());
    let mut error = url_params.as_ref().unwrap().get("error");

    if error.is_some() {
        return html!(error.unwrap());
    };

    yew::platform::spawn_local(async move {
        match (
            url_params.as_ref().unwrap().get("code"),
            url_params.as_ref().unwrap().get("state"),
        ) {
            (Some(code), Some(hash)) => {
                dispatch
                    .reduce_future(|state| async move {
                        let mut state = state.as_ref().clone();
                        state.auth_token = Some(code);
                        state.hash = hash;

                        log!(format!("{:?}", state));
                        let request =
                            gloo_net::http::Request::get("http://localhost:3001/getToken")
                                .query([("auth_token", state.auth_token.as_ref().unwrap())]);

                        let response = request.send().await;

                        match response {
                            Ok(response) => {
                                let token_response: TokenResponse = response
                                    .json::<TokenResponse>()
                                    .await
                                    .expect("Error: parsing token reponse json");

                                state.logged = true;
                                state.access_token = Some(token_response.access_token);
                                state.access_token_recieved = Some(chrono::Utc::now());
                                state.access_token_expires = Some(
                                    chrono::Utc::now()
                                        .checked_add_signed(
                                            chrono::Duration::from_std(
                                                std::time::Duration::from_secs(
                                                    token_response.expires_in,
                                                ),
                                            )
                                            .expect("Out of range"),
                                        )
                                        .unwrap(),
                                );
                            }
                            Err(err) => {
                                let error = Some("Error fetching spotify tokens".to_string());
                                log!(err.to_string())
                            }
                        }

                        Rc::new(state)
                    })
                    .await
            }
            (None, Some(hash)) => todo!(),
            (Some(code), None) => todo!(),
            (None, None) => todo!(),
        };
    });
    navigator.push(&crate::Route::Home);
    html!("fetching tokens")
}
