use std::{collections::HashMap, thread, time::Duration};

struct Cacher<T>
where
    T: Fn(u32) -> u32,
{
    calculator: T,
    value: HashMap<u32, u32>,
}

impl<T> Cacher<T>
where
    T: Fn(u32) -> u32,
{
    fn new(calculator: T) -> Cacher<T> {
        Cacher {
            calculator,
            value: HashMap::new(),
        }
    }

    fn value(&mut self, arg: u32) -> u32 {
        match self.value.get(&arg) {
            Some(v) => *v,
            None => {
                let v = (self.calculator)(arg);
                self.value.insert(arg, v);
                v
            }
        }
    }
}

fn expensive_function(num: u32) -> u32 {
    println!("calculating 2 slowly... for input {}", num);
    thread::sleep(Duration::from_secs(2));
    num
}

pub fn run() {
    println!("Closures!!!");

    let mut expensive_closure = Cacher::new(|num: u32| -> u32 {
        println!("calculating slowly... for input {}", num);
        thread::sleep(Duration::from_secs(2));
        num
    });

    let mut expensive_closure_2 = Cacher::new(expensive_function);

    let intensity = 35;

    if intensity < 25 {
        println!("call expensive operation twice");
        println!("1: input -> 1");
        expensive_closure.value(1);
        println!("2: input -> 1");
        expensive_closure.value(2);
    } else {
        println!("call expensive operation 3 times");
        println!("1: input -> 1");
        expensive_closure_2.value(1);
        println!("2: input -> 2");
        expensive_closure_2.value(2);
        println!("3: input -> 1");
        expensive_closure_2.value(1);
    }
}
