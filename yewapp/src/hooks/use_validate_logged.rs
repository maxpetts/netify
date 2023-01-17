use std::borrow::Cow;
use wasm_bindgen::prelude::wasm_bindgen;
use web_sys::{Event, EventTarget};
use yew::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_name = setTimeout)]
    fn timeout(timeout: u32);
}

#[hook]
pub fn use_validate_logged() {
    let navigator = yew_router::prelude::use_navigator().unwrap();
    let state = yewdux::prelude::use_store_value::<crate::State>();

    use_effect(move || {
        if state.access_token.is_none() {
            //or check timeout
            gloo_console::log!("not logged in");
            yew::platform::time::sleep(std::time::Duration::from_secs(3));
            navigator.push(&crate::Route::Login);
        }
    })
}
