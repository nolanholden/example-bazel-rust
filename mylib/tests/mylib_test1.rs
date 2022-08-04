extern crate mylib;

#[test]
fn test1_success() {
    assert_eq!("mylib world", mylib::world());
}

// Uncomment to test failure:
//
// #[test]
// fn test1_fail() {
//     assert_eq!("NOT mylib world", mylib::world());
// }
