
pub fn remove_between(text: &str, opening: char, closing: char) -> String {
	let mut new_text = String::new();
	let mut in_between = false;
	for c in text.chars() {
		if c == opening {
			in_between = true;
		} else if c == closing {
			in_between = false;
		} else if !in_between {
			new_text.push(c);
		}
	}

	// Make sure to remove tailing dashes if there are any
		new_text = new_text.trim_end_matches('-').to_string();

	return new_text;
}