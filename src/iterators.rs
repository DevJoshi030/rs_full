struct Counter {
    count: u32,
}

impl Counter {
    fn new() -> Counter {
        Counter { count: 0 }
    }
}

impl Iterator for Counter {
    type Item = u32;
    fn next(&mut self) -> Option<Self::Item> {
        if self.count < 5 {
            self.count += 1;
            Some(self.count)
        } else {
            None
        }
    }
}

pub fn run() {
    println!("Iterators!!!");

    let v1 = vec![1, 2, 3];

    let v1_iter = v1.iter();

    for value in v1_iter {
        println!("Value -> {}", value);
    }

    let v1_iter = v1.iter();

    let total: i32 = v1_iter.sum();

    println!("Total -> {}", total);

    let counter = Counter::new();

    for c in counter {
        println!("Counter -> {}", c);
    }

    let sum: u32 = Counter::new()
        .zip(Counter::new().skip(1))
        .map(|(a, b)| a * b)
        .filter(|x| x % 3 == 0)
        .sum();

    println!("Sum -> {}", sum);

    let check_zip = Counter::new().zip(Counter::new().skip(1));

    for (x, y) in check_zip {
        println!("x -> {}, y -> {}", x, y);
    }
}
