pub enum List<T>
where
    T: Display,
{
    Cons(T, Box<List<T>>),
    Nil,
}

impl<T> Display for List<T>
where
    T: Display,
{
    fn fmt(&self, _: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Cons(x, y) => match **y {
                Cons(..) => print!("{} -> {}", x, y),
                Nil => print!("{} -> END", x),
            },
            Nil => println!(),
        }
        Ok(())
    }
}

use std::fmt::Display;

use List::{Cons, Nil};

pub fn run() {
    println!("Smart Pointers!!!");

    let b = Box::new(5);

    println!("b -> {}", b);

    let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));

    println!("List = {}", list);
}
