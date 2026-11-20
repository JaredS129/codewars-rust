use std::collections::HashMap;

pub fn pete_the_baker(recipe: &HashMap<&str, u32>, available: &HashMap<&str, u32>) -> u32 {
    recipe
        .iter()
        .map(|(ingredient, &required)| available.get(ingredient).copied().unwrap_or(0) / required)
        .min()
        .unwrap_or(0)
}

#[cfg(test)]
mod tests;
