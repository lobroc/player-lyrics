use crate::musicfinder::{MprisReader, Song};
use mpris::DBusError;

pub trait UserInterface {
    fn display_song(&self, song_info: &Song, artists: &Vec<String>) -> Result<String, DBusError>;

    fn show_player(&self, mr: &MprisReader);

    fn display_verse(&mut self, verses: &Vec<&str>, position: usize);

    fn clear(&self);

    fn new() -> Self;
}
