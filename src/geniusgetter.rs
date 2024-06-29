use reqwest;
use scraper::{Html, Selector};

// Import utils module
use crate::utils;

pub fn build_url(artist: &str, song: &str) -> String {
	// Song needs to be all lower-case, and spaces turned to dashes
	let song_fmt = song.to_lowercase().replace(" ", "-");
	// Also need to remove anything between parentheses or brackets (find them, then delete what's between)
	let song_fmt = utils::remove_between(&song_fmt, '(', ')');
	let song_fmt = utils::remove_between(&song_fmt, '[', ']');
	// Also need to remove any special characters
	let song_fmt = song_fmt.replace("?", "").replace("!", "").replace(".", "").replace(",", "").replace("'", "").replace(":", "");
	// Artist need to be all lower-case, and spaces turned to dashes, but first character needs to be upper-case
	let artist = artist.to_lowercase().replace(" ", "-");
	let artist = format!("{}{}", artist.chars().next().unwrap().to_uppercase().to_string(), &artist[1..]);

	let url = format!("https://genius.com/{}-{}-lyrics", artist, song_fmt);

	return url;
}

pub fn get_lyrics(url: &str) -> String {
	let resp = reqwest::blocking::get(url).expect("Failed to get response");
	let body = resp.text().expect("Failed to get body");

	let document = Html::parse_document(&body);
	// Find the div with class containing substring "Lyrics__Container"
	let selector = Selector::parse("div[class*='Lyrics__Container']").unwrap();

	// For all matches to the selector, get the text and join them with newlines
	let mut lyrics = Vec::<String>::new();
	for element in document.select(&selector) {
		lyrics.push(element.text().collect::<Vec<_>>().join("\n"));
	}

	return lyrics.join("\n");
}