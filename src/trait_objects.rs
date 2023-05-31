use std::fmt::Debug;

trait Draw {
    fn draw(&self);
}

struct Screen {
    components: Vec<Box<dyn Draw>>,
}

impl Screen {
    fn display(&self) {
        for component in self.components.iter() {
            component.draw();
        }
    }
}

struct Button {
    width: u32,
    height: u32,
    label: String,
}

impl Draw for Button {
    fn draw(&self) {
        println!("Drawing a Button -> {:#?}", self);
    }
}

impl Debug for Button {
    fn fmt(&self, _: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        println!("Button {{");
        println!("  width: {},", self.width);
        println!("  height: {},", self.height);
        println!("  label: {},", self.label);
        println!("}}");

        Ok(())
    }
}

struct SelectBox {
    width: u32,
    height: u32,
    options: Vec<String>,
}

impl Draw for SelectBox {
    fn draw(&self) {
        println!("Drawing a SelectBox -> {:#?}", self);
    }
}

impl Debug for SelectBox {
    fn fmt(&self, _: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        println!("SelectBox {{");
        println!("  width: {},", self.width);
        println!("  height: {},", self.height);
        println!("  options: [");

        for opt in self.options.iter() {
            println!("      {},", opt);
        }

        println!("  ]");
        println!("}}");

        Ok(())
    }
}

pub fn run() {
    println!("Trait Objects!!!");

    let screen = Screen {
        components: vec![
            Box::new(Button {
                width: 100,
                height: 1,
                label: String::from("Click Me!!!"),
            }),
            Box::new(SelectBox {
                width: 200,
                height: 200,
                options: vec![
                    String::from("Yes"),
                    String::from("No"),
                    String::from("Maybe"),
                ],
            }),
        ],
    };

    screen.display();
}
