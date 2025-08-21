//!
//! Text-only, simple interface, which prints line-by-line to the terminal.
//! For more sophisticated visualisation, check [the TUI interface](./tui_interface.rs) or [the GTK interface](./gtk_interface.rs)
//!

use mpris::DBusError;

use crate::{
    musicfinder::{MprisReader, Song},
    user_interface::user_interface::UserInterface,
};

pub struct TextInterface {
    shown_lines: usize,
    last_shown_line: String,
}

impl UserInterface for TextInterface {
    fn new() -> Self {
        Self {
            shown_lines: 0usize,
            last_shown_line: String::new(),
        }
    }

    fn display_song(&self, song_info: &Song, artists: &Vec<String>) -> Result<String, DBusError> {
        let mut playing_status_str: String =
            format!("Now playing: {} by {}", song_info.title, song_info.artist);
        for a in artists[1..].iter() {
            playing_status_str.push_str(&format!("and {}", a));
        }
        println!("{}", playing_status_str);
        Ok(song_info.title.clone())
    }

    fn show_player(&self, mr: &MprisReader) {
        println!("Reading from player: {}", mr.get_player_name());
    }

    fn display_verse(&mut self, verses: &Vec<&str>, position: usize) {
        for idx in (self.shown_lines as usize)..=position {
            let fetched_verse: &str = match verses.get(idx) {
                Some(s) => s,
                None => {
                    continue;
                }
            };
            if fetched_verse != self.last_shown_line {
                println!("{}", fetched_verse);
                self.last_shown_line = fetched_verse.to_string();
            }
        }
        self.shown_lines = position;
    }

    fn clear(&self) {
        print!("{}[2J", 27 as char);
    }
}
