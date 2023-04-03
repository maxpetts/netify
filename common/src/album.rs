use serde::{Deserialize, Serialize};

use crate::{artist::SimpleArtist, image::Image, track::ExternalIds, user::ExternalUrls};

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct Restrictions {
    pub reason: String, // Allowed values: "market", "product", "explicit"
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct Copyright {
    pub text: String,
    pub r#type: char, // The type of copyright: C = the copyright, P = the sound recording (performance) copyright.
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct SimpleAlbum {
    pub album_group: String,
    pub album_type: String,
    pub artists: Vec<SimpleArtist>,
    pub available_markets: Vec<String>,
    pub external_urls: ExternalUrls,
    pub href: String,
    pub id: String,
    pub images: Vec<Image>,
    pub is_playable: bool,
    pub name: String,
    pub release_date: String,
    pub release_date_precision: String,
    pub restrictions: Option<Restrictions>,
    pub r#type: String,
    pub uri: String,
    pub total_tracks: u32,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct Album {
    pub album_type: String, // The type of the album. Allowed values: "album", "single", "compilation"
    pub total_tracks: u64,  // The number of tracks in the album.
    pub available_markets: Vec<String>, // The markets in which the album is available: ISO 3166-1
    pub external_urls: ExternalUrls,
    pub href: String, // A link to the Web API endpoint providing full details of the album.
    pub id: String,   // The Spotify ID for the album.
    pub images: Vec<Image>, // The cover art for the album in various sizes, widest first.
    pub name: String,
    pub release_date: String, // The date the album was first released.
    pub release_date_precision: String, // Allowed values: "year", "month", "day"
    pub restrictions: Option<Restrictions>,
    pub r#type: String, // The object type. Allowed values: "album"
    pub uri: String,
    pub copyrights: Option<Vec<Copyright>>,
    pub external_ids: ExternalIds,
    pub genres: Vec<String>,
    pub label: String,
    pub popularity: u8,
    pub album_group: String, // Allowed values: "album", "single", "compilation", "appears_on"
    pub artists: Vec<SimpleArtist>,
}
