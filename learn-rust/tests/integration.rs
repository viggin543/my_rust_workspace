// integration test uses a lib bublic methods. as an extenral user.
use ::mygrep; // use - just creates an alias to the name
mod common; // loads module into scope

#[test]
fn it_adds_two() {
    common::setup();
    assert_eq!(4, mygrep::add_two::add_two(2,2));
}