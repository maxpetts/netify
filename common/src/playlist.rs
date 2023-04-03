use crate::{
    image::Image,
    user::{Followers, PublicUser},
};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct PlaylistTracks {
    pub href: String,
    pub total: u32,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct FullPlaylist {
    pub collaborative: bool,
    pub description: Option<String>,
    pub external_urls: std::collections::HashMap<String, String>,
    // pub followers: Followers,
    pub href: String,
    pub id: String,
    pub images: Vec<Image>,
    pub name: String,
    pub owner: PublicUser,
    pub public: Option<bool>,
    pub snapshot_id: String,
    pub tracks: PlaylistTracks,
    pub r#type: String,
    pub uri: String,
}
