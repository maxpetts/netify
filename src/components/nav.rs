use leptos::*;
use leptos_router::*;

#[component]
pub fn Nav(cx: Scope) -> impl IntoView {
    let routes: Vec<Branch> = leptos_router::use_router(cx).possible_branches();
    // gloo_console::log!(routes);

    view! {cx, <p>"home"</p>}
}
