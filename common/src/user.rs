use serde::{Deserialize, Serialize};

use crate::image::Image;

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct Followers {
    pub href: Option<String>, // This will always be set to null, as the Web API does not support it at the moment.
    pub total: u32,           // The total number of followers.
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct ExternalUrls {
    pub spotify: String, // The Spotify URL for the object.
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct ExplicitContent {
    pub filter_enabled: Option<bool>,
    pub filter_locked: Option<bool>,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct PublicUser {
    pub display_name: Option<String>, // The name displayed on the user's profile. null if not available.
    pub uri: String,                  // The Spotify URI for this user.
    pub r#type: String,               // The object type. Allowed values: "user"
    pub id: String,                   // The Spotify user ID for this user.
    pub href: String,                 // A link to the Web API endpoint for this user.
    pub followers: Option<Followers>, // Information about the followers of this user.
    pub external_urls: ExternalUrls,  // Known public external URLs for this user.
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct Profile {
    pub display_name: Option<String>, // The name displayed on the user's profile. null if not available.
    pub uri: String,                  // The Spotify URI for this user.
    pub r#type: String,               // The object type. Allowed values: "user"
    pub id: String,                   // The Spotify user ID for this user.
    pub href: String,                 // A link to the Web API endpoint for this user.
    pub followers: Followers,         // Information about the followers of this user.
    pub external_urls: ExternalUrls,  // Known public external URLs for this user.
    pub country: String,
    pub product: String,
    pub images: Vec<Image>,
    pub explicit_content: ExplicitContent,
    pub email: String,
}
