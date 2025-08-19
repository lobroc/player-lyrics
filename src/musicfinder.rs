use core::time::Duration;
use mpris::{self, DBusError, FindingError};

pub struct Song {
    pub artist: String,
    pub title: String,
}

impl Song {
    fn new(artist: &str, title: &str) -> Self {
        Self {
            artist: String::from(artist),
            title: String::from(title),
        }
    }
}

pub struct MprisReader {
    player: mpris::Player,
}

impl MprisReader {
    pub fn new() -> Result<Self, FindingError> {
        let pf = match mpris::PlayerFinder::new() {
            Ok(p) => p,
            Err(e) => panic!("Failed to create MPRIS Reader: {e:}"),
        };
        let player: mpris::Player = mpris::PlayerFinder::find_active(&pf)?;
        Ok(Self { player })
    }

    pub fn get_player_name(&self) -> String {
        self.player.identity().to_string()
    }

    pub fn get_song(&self) -> Result<Song, DBusError> {
        // Get song name
        let meta: mpris::Metadata = self.player.get_metadata()?;
        let title_parsed: &str = match meta.title() {
            Some(n) => n,
            None => {
                return Err(DBusError::Miscellaneous(
                    "Failed to retrieve song title from MPRIS".to_string(),
                ));
            }
        };

        let artist_parsed: &str = match meta.artists() {
            Some(v) => v[0], // First artist = "most important"
            None => {
                return Err(DBusError::Miscellaneous(
                    "Failed to retrieve artist name from MPRIS".to_string(),
                ));
            }
        };

        Ok(Song::new(artist_parsed, title_parsed))
    }

    pub fn get_all_artists(&self) -> Result<Vec<String>, DBusError> {
        let meta: mpris::Metadata = self.player.get_metadata()?;
        let arts: Vec<String> = match meta.artists() {
            Some(v) => v.into_iter().map(|x| x.to_string()).collect(),
            None => {
                return Err(DBusError::Miscellaneous(
                    "Failed to retrieve artists from MPRIS".to_string(),
                ));
            }
        };
        Ok(arts)
    }

    pub fn song_progress(&self) -> Result<[Duration; 2], mpris::DBusError> {
        let mut progress_time: Duration = self.player.get_position()?;
        let track_length: Duration = match self.player.get_metadata()?.length() {
            Some(d) => d,
            None => Duration::ZERO,
        };
        // Skip forwards by 15 seconds, ensure that filler time (song beginning) is mostly ignored.
        // Worse to be too late than too early!
        progress_time += Duration::from_secs(15);
        Ok([progress_time, track_length])
    }
}
