use serde::{Deserialize, Serialize};

use crate::{
    image::Image,
    user::{ExternalUrls, Followers},
};

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct SimpleArtist {
    // pub external_urls: ExternalUrls,
    pub href: String,
    pub id: String,
    pub name: String,
    pub r#type: String, // Allowed values: "artist"
    pub uri: String,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct Artist {
    pub external_urls: ExternalUrls,
    pub followers: Followers,
    pub genres: Vec<String>,
    pub href: String,
    pub id: String,
    pub images: Vec<Image>,
    pub name: String,
    pub popularity: u8,
    pub r#type: String, // Allowed values: "artist"
    pub uri: String,
}
