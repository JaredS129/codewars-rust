fn get_numeric_position_in_alphabet(c: char) -> String {
    ((c.to_ascii_lowercase() as u8 - b'a') + 1).to_string()
}

pub fn alphabet_position(text: &str) -> String {
    let mut output: String = String::from("");
    text.trim()
        .chars()
        .filter(|c| c.is_ascii_alphabetic())
        .for_each(|c| {
            output.push_str(&get_numeric_position_in_alphabet(c));
            output.push(' ');
        });
    output.pop();
    output
}

#[cfg(test)]
mod tests;
