use spotify_types::playlist::FullPlaylist;
use styled_yew::styled;
use wasm_bindgen::JsCast;
use web_sys::HtmlElement;
use yew::{
    prelude::*,
    virtual_dom::{VChild, VTag},
};

#[derive(Properties, PartialEq)]
pub struct Props {
    pub playlist: FullPlaylist,
}
// border: "0.2em solid rgba(10,10,10,0.6)";
// background_color: "rgba(10,10,10,0.6)";
// border_radius: "1em";
// display: "flex";
// flex_direction: "column";
#[derive(Clone, Properties, PartialEq)]
struct DivProps {
    children: Children,
}

// styled!( pub Container : Div {
//     color: "red";
// });

styled!(pub Img : img {
    max_width: "35%";
});

#[function_component(Playlist)]
pub fn playlist(props: &Props) -> Html {
    let playlist = &props.playlist;

    // use_effect_with_deps(|_| {}, ());

    styled!(pub Bib : div {
        color: "green";
    });

    // #[function_component(Bib)]
    // fn div(props: &DivProps) -> Html {
    // let node_ref: NodeRef = use_node_ref();
    //     {
    //         let node_ref = node_ref.clone();
    //         use_effect_with_deps(
    //             |node_ref| {
    //                 let node = node_ref
    //                     .cast::<HtmlElement>()
    //                     .expect("node_ref not attached to element");

    //                 node.style()
    //                     .set_property("border", "3px solid red")
    //                     .unwrap();
    //             },
    //             node_ref,
    //         );
    //     }
    // let mut tag = VTag::new("div");
    // tag.node_ref = node_ref;
    // tag.add_children(props.children.clone().into_iter());

    //     html!({ tag })
    // }

    html! {
        <Bib>
            <h1>{&playlist.name}</h1>
            <Img src={playlist.images.first().unwrap().url.clone()}/>
        </Bib>
    }
}
