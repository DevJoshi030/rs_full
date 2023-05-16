use rand::Rng;
use std::{cmp::Ordering, process::exit};

#[derive(Debug)]
struct User {
    username: String,
    active: bool,
}

#[derive(Debug)]
pub struct Rect {
    pub width: u32,
    pub height: u32,
}

impl Rect {
    pub fn area(&self) -> u32 {
        self.width * self.height
    }

    pub fn can_hold(&self, other: &Rect) -> bool {
        self.width >= other.width && self.height >= other.height
    }

    pub fn square(size: u32) -> Rect {
        Rect {
            width: size,
            height: size,
        }
    }
}

pub fn run() {
    let mut user1 = User {
        active: true,
        username: String::from("elgaeraw"),
    };

    let name = user1.username;
    let active = user1.active;

    println!("{} -> {}", name, active);

    user1.username = String::from("elgaEraW");

    println!("{}", name);

    let user2 = build_user(String::from("elagEraW123"));

    println!("{:?}", user2);

    let rect = Rect {
        width: 30,
        height: 50,
    };

    let rect2 = Rect {
        width: 10,
        height: 50,
    };

    let rect3 = Rect {
        width: 30,
        height: 60,
    };

    let sq = Rect::square(40);

    println!("{}", rect.area());
    println!("{:#?}", sq);

    println!("can_hold: rect -> rect2 {}", rect.can_hold(&rect2));
    println!("can_hold: rect -> rect3 {}", rect.can_hold(&rect3));

    let a = 0.1;
    let b = 0.2;
    let c = 0.3;

    if c == a + b {
        println!("correct");
    } else {
        println!("faltu");
    }

    println!("Guess the number:");

    let secret = rand::thread_rng().gen_range(1..101);
    println!("{secret}");

    for _ in 0..5 {
        let mut guess = String::new();
        std::io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(g) => g,
            Err(_) => {
                println!("failed to parse");
                continue;
            }
        };
        match guess.cmp(&secret) {
            Ordering::Equal => {
                println!("Correct");
                exit(0);
            }
            Ordering::Greater => {
                println!("Too High");
            }
            Ordering::Less => {
                println!("Too Low");
            }
        }
    }

    println!("Couldn't guess...sry...correct ans: {secret}");
}

fn build_user(username: String) -> User {
    User {
        username,
        active: true,
    }
}
