// use std::rc::Rc;

use serde::{Deserialize, Serialize};

use crate::{
    album::{Restrictions, SimpleAlbum},
    artist::{Artist, SimpleArtist},
    user::ExternalUrls,
};

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct ExternalIds {
    pub isrc: Option<String>,
    pub ean: Option<String>,
    pub upc: Option<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct Track {
    pub album: SimpleAlbum,
    pub artists: Vec<SimpleArtist>,
    pub available_markets: Vec<String>, // ISO 3166-1 alpha-2
    pub disc_number: u8,
    pub duration_ms: u64, // The track length in milliseconds.
    pub explicit: bool,
    pub external_ids: ExternalIds,
    pub external_urls: ExternalUrls,
    pub href: String,
    pub id: String,
    pub is_playable: Option<bool>,
    pub linked_from: Option<Box<Track>>,
    pub restrictions: Option<Restrictions>,
    pub name: String,
    pub popularity: u8,
    pub preview_url: String,
    pub track_number: u16,
    pub r#type: String,
    pub uri: String,
    pub is_local: bool,
}
