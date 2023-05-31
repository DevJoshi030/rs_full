pub struct AveragedCollection {
    list: Vec<i32>,
    avg: f64,
}

impl AveragedCollection {
    pub fn new() -> AveragedCollection {
        AveragedCollection {
            list: vec![],
            avg: 0 as f64,
        }
    }

    pub fn add(&mut self, value: i32) {
        self.list.push(value);
        self.update_avg();
    }

    pub fn pop(&mut self) -> Option<i32> {
        let value = self.list.pop();

        match value {
            Some(v) => {
                self.update_avg();
                Some(v)
            }
            None => None,
        }
    }

    pub fn avg(&self) -> f64 {
        self.avg
    }

    pub fn update_avg(&mut self) {
        self.avg = self.list.iter().sum::<i32>() as f64 / self.list.len() as f64;
    }
}

pub fn run() {
    println!("Object Oriented!!!");
    let mut a = AveragedCollection::new();
    a.add(1);
    a.add(2);
    a.add(3);
    println!("avg -> {}", a.avg());
    a.pop();
    println!("avg -> {}", a.avg());
}
