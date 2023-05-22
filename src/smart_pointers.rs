use std::{cell::RefCell, fmt::Display, ops::Deref, rc::Rc};

use List::{Cons, Nil};
enum List<T>
where
    T: Display,
{
    Cons(Rc<RefCell<T>>, Rc<List<T>>),
    Nil,
}

impl<T> Display for List<T>
where
    T: Display,
{
    fn fmt(&self, _: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Cons(x, y) => match **y {
                Cons(..) => {
                    print!("{} -> {}", *x.deref().borrow(), y)
                }
                Nil => print!("{} -> END", *x.deref().borrow()),
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

    let list = Cons(
        Rc::new(RefCell::new(1)),
        Rc::new(Cons(
            Rc::new(RefCell::new(2)),
            Rc::new(Cons(Rc::new(RefCell::new(3)), Rc::new(Nil))),
        )),
    );

    println!("List = {}", list);

    let x = 5;
    let y = MyBox::new(x);

    println!("x -> {} and y -> {}", x, *y);

    let a = Rc::new(Cons(
        Rc::new(RefCell::new(5)),
        Rc::new(Cons(Rc::new(RefCell::new(10)), Rc::new(Nil))),
    ));
    println!("a -> {}", a);
    println!("Reference Counter: {}", Rc::strong_count(&a));
    let b = Cons(Rc::new(RefCell::new(3)), Rc::clone(&a));
    println!("b -> {}", b);
    println!("Reference Counter: {}", Rc::strong_count(&a));
    {
        let c = Cons(Rc::new(RefCell::new(4)), Rc::clone(&a));
        println!("c -> {}", c);
        println!("Reference Counter: {}", Rc::strong_count(&a));
    }
    println!("Reference Counter: {}", Rc::strong_count(&a));
}
