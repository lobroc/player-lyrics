mod geniusgetter;
mod musicfinder;
mod utils;

use musicfinder::{MprisReader, Song};
use std::error::Error;
use std::time::Duration;

fn main() -> Result<(), Box<dyn Error>> {
    let mr: MprisReader = MprisReader::new()?;
    println!("Reading from player: {}", mr.get_player_name());
    let mut last_song = String::new(); // Dummy value
    let mut lyrics = String::new();
    let mut shown_lines: usize = 0;
    let mut last_shown_line: String = String::new();

    // Infinite loop
    loop {
        // Check every 5 seconds
        std::thread::sleep(Duration::from_secs(1));
        let songinfo: Song = mr.get_song()?;

        if songinfo.title == "Up next" {
            continue;
        }

        if last_song != songinfo.title {
            let artists: Vec<String> = mr.get_all_artists()?;

            let mut playing_status_str: String =
                format!("Now playing: {} by {}", songinfo.title, songinfo.artist);
            for a in artists[1..].iter() {
                playing_status_str.push_str(&format!("and {}", a));
            }
            last_song = songinfo.title.clone();
            println!("{}", playing_status_str);

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
            let mut verses_cleaned: Vec<&str> = vec![];
            for v in verses {
                let firstchar: char = match v.chars().next() {
                    Some(c) => c,
                    None => {
                        continue;
                    }
                };
                if !(firstchar == '[' || firstchar == '(' || firstchar == ')') {
                    verses_cleaned.push(v);
                }
            }

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
            for idx in shown_lines..=position {
                let fetched_verse: &str = match verses_cleaned.get(idx) {
                    Some(s) => s,
                    None => {
                        continue;
                    }
                };
                if fetched_verse != last_shown_line {
                    println!("{}", fetched_verse);
                    last_shown_line = fetched_verse.to_string();
                }
            }
            shown_lines = position;
        }
    }
}
