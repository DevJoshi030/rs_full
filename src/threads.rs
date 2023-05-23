use std::{thread, time::Duration};

pub fn run() {
    println!("Threads!!!");

    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("i (spawned) -> {}", i);
            thread::sleep(Duration::from_millis(100));
        }
    });

    handle.join().unwrap();

    for i in 1..5 {
        println!("i (main) -> {}", i);
        thread::sleep(Duration::from_millis(50));
    }

    let v = vec![1, 2, 3];

    let handle2 = thread::spawn(move || {
        println!("v -> {:#?}", v);
    });

    handle2.join().unwrap();
}
