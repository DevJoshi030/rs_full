use std::fmt::Display;

#[derive(Debug)]
struct ImportantExcerpt<'a> {
    part: &'a str,
}

pub fn run() {
    println!("Lifetimes!!!");

    let str1 = String::from("a");
    let str2 = String::from("b");

    let result = longest(str1.as_str(), str2.as_str());

    println!("{}", result);

    let i;

    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel.split('.').next().expect("Could not find a '.'");
    i = ImportantExcerpt {
        part: first_sentence,
    };

    println!("{:#?}", i.part);

    let result = longest_with_announcement(str1.as_str(), str2.as_str(), "Test Announcement!!");
    println!("{}", result);
}

fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

fn longest_with_announcement<'a, T: Display>(x: &'a str, y: &'a str, ann: T) -> &'a str {
    println!("{}", ann);

    if x.len() > y.len() {
        x
    } else {
        y
    }
}
