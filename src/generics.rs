#[derive(Debug)]
struct Point<T, U> {
    x: T,
    y: U,
}

impl<T, U> Point<T, U> {
    fn mixup<V, W>(self, other: Point<V, W>) -> Point<T, W> {
        Point {
            x: self.x,
            y: other.y,
        }
    }
}

pub fn run() {
    println!("Generics!!!");
    let number_list = vec![10, 20, 50, 30, 40];

    let largest = get_largest(number_list);

    println!("{}", largest);

    let char_list = vec!['y', 'a', 'g', 'z', 'f'];

    let largest = get_largest(char_list);

    println!("{}", largest);

    let p1 = Point { x: 2, y: 5 };
    let p2 = Point { x: 5.6, y: 8.5 };

    let p3 = p1.mixup(p2);

    println!("{:#?}", p3);
}

fn get_largest<T: PartialOrd + Copy>(list: Vec<T>) -> T {
    let mut largest = *list.get(0).unwrap();

    for item in list {
        if item > largest {
            largest = item;
        }
    }

    return largest;
}
