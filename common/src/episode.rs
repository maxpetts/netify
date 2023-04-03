use serde::{Deserialize, Serialize};

use crate::{album::Restrictions, image::Image, show::SimplifiedShow, user::ExternalUrls};

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct ResumePoint {
    pub fully_played: bool,
    pub resume_position_ms: String,
}
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct Episode {
    pub audio_preview_url: String,
    pub description: String,
    pub html_description: String,
    pub duration_ms: String,
    pub explicit: bool,
    pub external_urls: ExternalUrls,
    pub href: String,
    pub id: String,
    pub images: Vec<Image>,
    pub is_externally_hosted: bool,
    pub is_playable: bool,
    pub languages: Vec<String>, // A list of the languages used in the episode, identified by their ISO 639-1 code.
    pub name: String,
    pub release_date: String,
    pub release_date_precision: String,
    pub resume_point: ResumePoint,
    pub r#type: String,
    pub uri: String,
    pub restrictions: Restrictions,
    pub show: SimplifiedShow,
}
