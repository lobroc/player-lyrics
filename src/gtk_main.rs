mod geniusgetter;
mod musicfinder;
mod user_interface;
mod utils;

use gtk4::glib::{self, idle_add_local, timeout_add_seconds_local};
use musicfinder::{MprisReader, Song};
use std::error::Error;
use std::time::Duration;

use crate::user_interface::gtk_interface::GTKInterface;
use crate::user_interface::user_interface::UserInterface;

fn main() -> Result<glib::ExitCode, Box<dyn Error>> {
    let mr = MprisReader::new()?;
    let mut iface = GTKInterface::new();
    let iface_clone: GTKInterface = iface.clone();

    let mut last_song = String::new(); // Dummy value
    let mut lyrics = String::new();

    {
        // Scoped cloning for submission only
        let iface_clone2: GTKInterface = iface.clone();
        let mr_clone2 = mr.clone();
        idle_add_local(move || {
            iface_clone2.show_player(&mr_clone2);
            glib::ControlFlow::Break
        });
    }

    // Infinite loop, called every second
    timeout_add_seconds_local(1, move || {
        let songinfo: Song = match mr.get_song() {
            Ok(s) => s,
            Err(e) => {
                eprintln!("Error: {e:}");
                return glib::ControlFlow::Break;
            }
        };

        if songinfo.title == "Up next" {
            return glib::ControlFlow::Continue;
        }

        if last_song != songinfo.title {
            iface.clear();

            let artists: Vec<String> = match mr.get_all_artists() {
                Ok(a) => a,
                Err(e) => {
                    eprintln!("Error: {e:}");
                    return glib::ControlFlow::Break;
                }
            };

            last_song = match iface.display_song(&songinfo, &artists, &mr) {
                Ok(s) => s,
                Err(e) => {
                    eprintln!("Error: {e:}");
                    return glib::ControlFlow::Break;
                }
            };

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
                            return glib::ControlFlow::Continue;
                        }
                    }
                }
            };
            iface.append_text_to_ui("Lyrics:");
        } else {
            if lyrics.is_empty() {
                return glib::ControlFlow::Continue;
            }

            // Here, we update current lyrics
            let [playback_pos, length] = match mr.song_progress() {
                Ok(p) => p,
                Err(e) => {
                    eprintln!("Error: {e:}");
                    return glib::ControlFlow::Break;
                }
            };
            let verses: Vec<&str> = lyrics.lines().collect();
            let verses_cleaned: Vec<String> = utils::genius_lyrics_cleaner(verses);
            let verses_cleaned_slice: Vec<&str> =
                verses_cleaned.iter().map(|x| x.as_str()).collect();

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
            iface.display_verse(&verses_cleaned_slice, position);
        }
        glib::ControlFlow::Continue
    });

    eprintln!("Launching iface!");
    iface_clone.launch();

    Ok(glib::ExitCode::new(0))
}
