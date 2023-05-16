use rs_full::misc::Rect;
mod common;

#[test]
fn it_works() {
    common::setup();
    assert_eq!(2 + 2, 4);
}

#[test]
fn larger_can_hold_smaller() {
    let rect1 = Rect {
        width: 90,
        height: 30,
    };

    let rect2 = Rect {
        width: 60,
        height: 30,
    };

    assert!(rect1.can_hold(&rect2));
}

#[test]
#[ignore = "expensive"]
fn expensive_test() {
    // We can run this test manually with
    // -> cargo test -- --include-ignored
    // or with ->
    // cargo test -- --ignored (only run ignored tests)
    println!("Very Expensive Test...")
}
