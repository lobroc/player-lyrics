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

pub fn genius_lyrics_cleaner(verses: Vec<&str>) -> Vec<String> {
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

    let mut result = String::new();
    let mut prev_line_was_normal = false;

    for line in verses_cleaned {
        let line_trimmed = line.trim();

        if line_trimmed.is_empty() {
            // Collapse consecutive empty lines into a single '\n'
            if prev_line_was_normal {
                result.push('\n');
                prev_line_was_normal = false;
            }
            continue;
        }

        if line_trimmed.len() < 5 {
            // Short line → append directly to current line (no newline)
            result.push_str(line_trimmed);
        } else {
            // Normal line
            if !result.is_empty() && prev_line_was_normal {
                result.push('\n');
            }
            result.push_str(line_trimmed);
            prev_line_was_normal = true;
        }
    }
    result.lines().map(|x| x.to_string()).collect()
}
