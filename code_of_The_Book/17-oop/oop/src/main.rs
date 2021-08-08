
trait Draw {
    fn draw(&self);
}

struct Screen {
    components: Vec<Box<dyn Draw>> // vec 中可以是 Button 或者 SelectBox, 只要满足了 Draw trait 就可以.
}

impl Screen {
    fn run(&self) {
        for component in self.components.iter() {
            component.draw();
        }
    }
}

struct Button {

}

impl Draw for Button {
    fn draw(&self) {
        println!("Draw a Button.");
    }
}

struct SelectBox {

}

impl Draw for SelectBox {
    fn draw(&self) {
        println!("Draw a SelectBox");
    }
}

fn main() {
    
    let screen = Screen {
        components: vec![
            Box::new(Button{

            }),
            Box::new(SelectBox {

            })
        ]
    };

    screen.run();
}
