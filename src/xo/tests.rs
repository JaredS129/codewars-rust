use super::*;

fn do_test(s: &str, expected: bool) {
    let actual = xo(s);
    assert!(
        expected == actual,
        "Test failed.\n\nInput:    {s:?}\nExpected: {expected}\nActual:   {actual}\n"
    )
}

#[test]
fn basic_tests() {
    do_test("xo", true);
    do_test("Xo", true);
    do_test("xxOo", true);
    do_test("xxxm", false);
    do_test("Oo", false);
    do_test("ooom", false);
}
