use rs_full::grep::Line;

#[test]
fn one_result() {
    let query = "duct";
    let contents = "\
Rust:
safe, fast, productive.
Pick three.";

    assert_eq!(
        vec![Line {
            index: 2,
            text: "safe, fast, productive."
        }],
        rs_full::grep::search(query, contents)
    );
}
