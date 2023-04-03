use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub children: Children,
    pub style: Option<stylist::StyleSource>,
    pub onclick: Callback<yew::MouseEvent>,
}

#[function_component(Button)]
pub fn button(props: &Props) -> Html {
    let style = stylist::css!(
        r#"
        border-radius: 1em;
        background-color: #111;
        color: white;
    "#
    );

    // let label = props.label.clone();

    html! {
        <button class={if props.style.is_some() {props.style.clone().unwrap()} else {style}} onclick={&props.onclick}>{for props.children.iter()}</button>
    }
}
