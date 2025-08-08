use crate::utils;
use reqwest;
use scraper::{Html, Selector};

#[derive(Debug)]
pub struct ExtractionError {
    message: &'static str,
}

impl ExtractionError {
    fn new(message: &'static str) -> ExtractionError {
        Self { message }
    }
}

impl std::fmt::Display for ExtractionError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", self.message)
    }
}

impl std::error::Error for ExtractionError {}

pub fn build_url(artists: Vec<String>, song: &str, all_artists: bool) -> String {
    // Song needs to be all lower-case, and spaces turned to dashes
    let song_fmt: String = song.to_lowercase().replace(" ", "-");
    // Also need to remove anything between parentheses or brackets (find them, then delete what's between)
    let song_fmt: String = utils::remove_between(&song_fmt, '(', ')');
    let song_fmt: String = utils::remove_between(&song_fmt, '[', ']');
    // Also need to remove any special characters
    let mut song_cleaned: Vec<char> = Vec::with_capacity(song_fmt.len());
    for c in song_fmt.chars() {
        if c.is_alphanumeric() || (c == '-') {
            song_cleaned.push(c);
        }
    }
    let song_fmt: String = song_cleaned.iter().cloned().collect();
    // Artist need to be all lower-case, and spaces turned to dashes, but first character needs to be upper-case
    let artist = artists[0].to_lowercase().replace(" ", "-");
    let artist = format!("{}{}", artist[0..1].to_uppercase(), &artist[1..]);

    let mut url = String::new();
    if all_artists {
        url.push_str("https://genius.com/");
        url.push_str(&artist);
        for art in artists[1..].iter() {
            url.push_str("-and-");
            url.push_str(&art);
        }
        url.push_str(&format!("-{}-lyrics", song_fmt));
    } else {
        url = format!("https://genius.com/{}-{}-lyrics", artist, song_fmt);
    }

    while url.contains("--") {
        url = url.replace("--", "-");
    }

    return url;
}

pub fn get_lyrics(url: &str) -> Result<String, ExtractionError> {
    let resp = reqwest::blocking::get(url).expect("Failed to get response");
    let body = resp.text().expect("Failed to get body");

    if body.contains("Oops! Page not found") {
        return Err(ExtractionError::new(
            "Extraction has failed: is this an 'official' song release (leak)? Otherwise, please report bugs!",
        ));
    }

    let document = Html::parse_document(&body);
    // Find the div with class containing substring "Lyrics__Container"
    let selector = Selector::parse("div[class*='Lyrics__Container']")
        .expect("Failed to retrieve Lyrics from web page");

    // For all matches to the selector, get the text and join them with newlines
    let mut lyrics = String::new();
    for element in document.select(&selector) {
        lyrics.push_str(&element.text().collect::<Vec<_>>().join("\n"));
        lyrics.push('\n');
    }
    let bloat_words: [&str; 4] = ["Contributors", "Translations", "Lyrics", "[Intro]"]; // The are words that mark we are still in the preamble
    for w in bloat_words {
        lyrics = match lyrics.find(w) {
            Some(idx) => lyrics[idx..].to_string(),
            None => lyrics,
        };
    }

    return Ok(lyrics);
}
