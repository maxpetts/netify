pub mod actions;
pub mod album;
pub mod artist;
pub mod curr_playing;
pub mod device;
pub mod episode;
pub mod image;
pub mod page;
pub mod playlist;
pub mod show;
pub mod track;
pub mod user;

trait Playable {
    fn play(&self);
    fn seek_and_play(&self, position_ms: u32);
}
