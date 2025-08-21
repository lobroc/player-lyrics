use crate::musicfinder::{MprisReader, Song};
use mpris::DBusError;

pub trait UserInterface {
    ///
    /// Prints song information, and returns song title.
    ///
    fn display_song(&self, song_info: &Song, artists: &Vec<String>) -> Result<String, DBusError>;

    fn show_player(&self, mr: &MprisReader);

    ///
    /// This function requires the instance to be mutable.
    /// Verses should be a line-by-line vector of string slices.
    /// Position is an int representing the index at which we are currently located / playing.
    ///
    fn display_verse(&mut self, verses: &Vec<&str>, position: usize);

    fn clear(&self);

    fn new() -> Self;
}
