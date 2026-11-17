pub fn find_odd(arr: &[i32]) -> i32 {
    let mut sorted = Vec::from(arr);
    sorted.sort();

    let len = sorted.len();

    let mut i: usize = 0;

    loop {
        if i == len - 1 {
            return sorted[i];
        }
        if sorted[i] != sorted[i + 1] {
            return sorted[i];
        }
        i += 2;
    }
}

#[cfg(test)]
mod tests;
