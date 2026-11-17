fn get_count_of_char_in_str(c: char, s: &str) -> usize {
    s.to_lowercase().chars().filter(|c2| *c2 == c).count()
}

pub fn xo(s: &str) -> bool {
    get_count_of_char_in_str('x', s) == get_count_of_char_in_str('o', s)
}

#[cfg(test)]
mod tests;
