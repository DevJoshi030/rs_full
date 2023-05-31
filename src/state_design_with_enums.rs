enum State {
    Draft,
    PendingReview,
    Published,
}

struct Post {
    state: State,
    content: String,
}

impl Post {
    fn new() -> Post {
        Post {
            state: State::Draft,
            content: String::from(""),
        }
    }

    pub fn add_text(&mut self, text: &str) {
        self.content.push_str(text);
    }

    pub fn content(&self) -> &str {
        match self.state {
            State::Published => &self.content,
            _ => "",
        }
    }

    pub fn request_review(&mut self) {
        match self.state {
            State::Draft => self.state = State::PendingReview,
            _ => {}
        }
    }

    pub fn approve(&mut self) {
        match self.state {
            State::PendingReview => self.state = State::Published,
            _ => {}
        }
    }
}

pub fn run() {
    println!("State Design With Enums!!!");
    let mut post = Post::new();
    post.add_text("Sample Text");
    println!("Post Content -> {}", post.content());
    post.request_review();
    println!("Post Content -> {}", post.content());
    post.approve();
    println!("Post Content -> {}", post.content());
}
