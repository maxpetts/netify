use serde::{Deserialize, Serialize};

use crate::{actions::Actions, device::Device, episode::Episode, track::Track, user::ExternalUrls};

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct Context {
    pub r#type: String, // The object type, e.g. "artist", "playlist", "album", "show".
    pub href: String,   // A link to the Web API endpoint providing full details of the track.
    pub external_urls: ExternalUrls, // External URLs for this context.
    pub uri: String,    // The Spotify URI for the context.
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(untagged)]
pub enum PlayableItem {
    Track(Track),
    Episode(Episode),
}

impl PlayableItem {
    pub fn name(&self) -> &str {
        match self {
            PlayableItem::Track(t) => &t.name,
            PlayableItem::Episode(e) => &e.name,
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct CurrentlyPlaying {
    pub device: Option<Device>,
    pub repeat_state: Option<String>,
    pub shuffle_state: Option<bool>,
    pub context: Option<Context>,
    pub timestamp: u64, // Unix Millisecond Timestamp when data was fetched.
    pub progress_ms: Option<u64>, // Progress into the currently playing track or episode. Can be null.
    pub is_playing: bool,         // If something is currently playing, return true.
    pub item: Option<PlayableItem>, // The currently playing track or episode. Can be null.
    // pub item: Option<Track>, // The currently playing track or episode. Can be null.
    pub currently_playing_type: String,
    pub actions: Actions,
}
