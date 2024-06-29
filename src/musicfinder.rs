use mpris;

pub struct Song {
	pub artist: String,
	pub song: String
}

pub fn get_player_name(verbose: bool) -> String {
	let pf = mpris::PlayerFinder::new().expect("Failed to create player finder");

    let player = mpris::PlayerFinder::find_active(&pf).expect("Failed to find active player");

    // Get player name
    let player_name = player.identity();
	if verbose {
    	println!("Active media player: {}", player_name);
	}

	return player_name.to_string();

}

pub fn get_song(verbose: bool) -> Song {
	let pf = mpris::PlayerFinder::new().expect("Failed to create player finder");

	let player = mpris::PlayerFinder::find_active(&pf).expect("Failed to find active player");

	// Get song name
	let meta = player.get_metadata().expect("Failed to get metadata");
	let song_parsed = meta.title().expect("Failed to read song").to_string();
	if verbose {
		println!("Now playing: {}", song_parsed);
	}
	let artist_parsed = meta.artists().expect("Failed to read artist")[0].to_string();

	// Give both artist and song name, as struct
	return Song {
		artist: artist_parsed,
		song: song_parsed
	};
}