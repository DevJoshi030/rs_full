use rs_full::misc::Rect;

#[test]
fn it_works() {
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
