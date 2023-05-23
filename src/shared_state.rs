use std::{
    sync::{Arc, Mutex},
    thread,
    time::Duration,
};

pub fn run() {
    println!("Shared State!!!");

    let m = Mutex::new(5);
    {
        let mut num = m.lock().unwrap();

        *num = 10;
    }
    println!("m -> {:?}", m);

    let counter = Arc::new(Mutex::new(0));

    let mut handles = vec![];

    for _ in 0..10 {
        let counter_clone = Arc::clone(&counter);
        handles.push(thread::spawn(move || {
            let mut num = counter_clone.lock().unwrap();
            *num += 1;
            thread::sleep(Duration::from_millis(100));
        }));
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Counter -> {}", *counter.lock().unwrap());
}
