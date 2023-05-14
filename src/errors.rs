use std::{fs::File, io::Read};

pub fn run() {
    // panic!("test panic");

    let mut f = File::open("src/misc.rs").expect("Failed to open the file");

    let mut s = String::new();
    f.read_to_string(&mut s).expect("failed to read");

    println!("{}", s);
}
