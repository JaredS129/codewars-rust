use super::*;
#[test]
fn test_abcde() {
    assert_eq!(count_duplicates("abcde"), 0);
}

#[test]
fn test_abcdea() {
    assert_eq!(count_duplicates("abcdea"), 1);
}

#[test]
fn test_indivisibility() {
    assert_eq!(count_duplicates("indivisibility"), 1);
}
