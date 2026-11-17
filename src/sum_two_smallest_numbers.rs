pub fn sum_two_smallest_numbers(numbers: &[u32]) -> u32 {
    let mut input_vec = Vec::from(numbers);
    input_vec.sort();
    input_vec[0] + input_vec[1]
}

#[cfg(test)]
mod tests;
