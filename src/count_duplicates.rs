use std::collections::HashSet;

pub fn count_duplicates(text: &str) -> u32 {
    let chars: Vec<char> = text.to_lowercase().chars().collect();
    let char_set: HashSet<char> = chars.iter().copied().collect();
    let mut distinct_chars: u32 = 0;

    for char in &char_set {
        let mut char_count: u8 = 0;
        for c in &chars {
            if c == char {
                char_count += 1;
                if char_count > 1 {
                    distinct_chars += 1;
                    break;
                }
            }
        }
    }
    distinct_chars
}

#[cfg(test)]
mod tests;
