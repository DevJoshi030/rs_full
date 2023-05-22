use std::{fmt::Display, ops::Deref};

use List::{Cons, Nil};
enum List<T>
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
            _ => (),
        }
        Ok(())
    }
}

struct MyBox<T: Display>(T);

impl<T> MyBox<T>
where
    T: Display,
{
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

impl<T> Deref for MyBox<T>
where
    T: Display,
{
    type Target = T;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T> Drop for MyBox<T>
where
    T: Display,
{
    fn drop(&mut self) {
        println!("Dropping the MyBox: {}", &self.0);
    }
}

pub fn run() {
    println!("Smart Pointers!!!");

    let b = Box::new(5);

    println!("b -> {}", b);

    let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));

    println!("List = {}", list);

    let x = 5;
    let y = MyBox::new(x);

    println!("x -> {} and y -> {}", x, *y);
}
