pub fn binary_slice_to_number(slice: &[u32]) -> u32 {
    if slice.iter().any(|num| *num != 0 && *num != 1) {
        panic!();
    };
    if slice.is_empty() {
        panic!();
    };

    let input_as_string: String = slice
        .iter()
        .map(|num| num.to_string().chars().next().unwrap())
        .collect();

    u32::from_str_radix(&input_as_string, 2).unwrap()
}

#[cfg(test)]
mod tests;
