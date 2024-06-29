mod musicfinder;
mod geniusgetter;
mod utils;

fn main() {
    println!("Reading from player: {}", musicfinder::get_player_name(false));
    let mut last_song = "AAAAAAAAAAAAAA".to_string(); // Dummy value

    // Infinite loop
    loop {
        // Check every 5 seconds
        std::thread::sleep(std::time::Duration::from_secs(5));
        let songinfo = musicfinder::get_song(false);
        if last_song != songinfo.song {
            println!("Now playing: {} by {}", songinfo.song, songinfo.artist);
            let url = geniusgetter::build_url(&songinfo.artist, &songinfo.song);
            println!("Getting lyrics from: {}", url);
            let lyrics = geniusgetter::get_lyrics(&url);
            println!("Lyrics: {}", lyrics);
            last_song = songinfo.song.clone();
        }
    }

}
