use std::{sync::mpsc, thread, time::Duration};

pub fn run() {
    println!("Message Passing!!!");

    let (tx, rx) = mpsc::channel();

    let tx2 = tx.clone();

    thread::spawn(move || {
        let vals = vec!["t1-m1", "t1-m2", "t1-m3"];

        for msg in vals {
            tx.send(msg).unwrap();
            thread::sleep(Duration::from_millis(500));
        }
    });

    thread::spawn(move || {
        let vals = vec!["t2-m1", "t2-m2", "t2-m3"];

        for msg in vals {
            tx2.send(msg).unwrap();
            thread::sleep(Duration::from_millis(500));
        }
    });
    for received in rx {
        println!("received -> {}", received);
    }
}
