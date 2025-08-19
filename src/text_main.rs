mod geniusgetter;
mod musicfinder;
mod user_interface;
mod utils;

use musicfinder::{MprisReader, Song};
use std::error::Error;
use std::time::Duration;

use crate::user_interface::text_interface::TextInterface;
use crate::user_interface::user_interface::UserInterface;

fn main() -> Result<(), Box<dyn Error>> {
    let mr = MprisReader::new()?;
    let mut iface = TextInterface::new();

    iface.show_player(&mr);

    let mut last_song = String::new(); // Dummy value
    let mut lyrics = String::new();

    // Infinite loop
    loop {
        // Check every 5 seconds
        std::thread::sleep(Duration::from_secs(1));
        let songinfo: Song = mr.get_song()?;

        if songinfo.title == "Up next" {
            continue;
        }

        if last_song != songinfo.title {
            iface.clear();

            let artists: Vec<String> = mr.get_all_artists()?;

            last_song = iface.display_song(&songinfo, &artists)?;

            let url: String = geniusgetter::build_url(artists.clone(), &songinfo.title, false);
            println!("Getting lyrics from: {}", url);
            lyrics = match geniusgetter::get_lyrics(&url) {
                Ok(lyr) => lyr,
                Err(_e) => {
                    eprintln!("First extraction failed, trying again with all artist names.");
                    let url = geniusgetter::build_url(artists, &songinfo.title, true); // This time, try with all artists!
                    println!("Getting lyrics from: {}", url);
                    match geniusgetter::get_lyrics(&url) {
                        Ok(lyr) => lyr,
                        Err(e) => {
                            eprintln!("Extraction failed again. Giving up. Error: {e:}");
                            continue;
                        }
                    }
                }
            };
            println!("Lyrics:");
        } else {
            if lyrics.is_empty() {
                continue;
            }

            // Here, we update current lyrics
            let [playback_pos, length] = mr.song_progress()?;
            let verses: Vec<&str> = lyrics.lines().collect();
            let verses_cleaned: Vec<&str> = utils::genius_lyrics_cleaner(verses);

            let mut position: usize;
            match length {
                Duration::ZERO => {
                    position = (playback_pos.as_secs_f32() / 2.0) as usize; // Assume 2 seconds per verse. Not ideal, but best I can do.
                    if position > verses_cleaned.len() {
                        position = verses_cleaned.len() - 1; // Clip if over
                    }
                }
                _ => {
                    position = ((playback_pos.as_secs_f32() / length.as_secs_f32())
                        * verses_cleaned.len() as f32) as usize;
                }
            }
            iface.display_verse(&verses_cleaned, position);
        }
    }
}
