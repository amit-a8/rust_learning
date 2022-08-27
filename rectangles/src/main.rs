#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };
    let rect2 = Rectangle{
        width: 40,
        height : 60,
    };

    println!(
        "The area of the rectangle is {} square pixels. {}",
        rect1.area(), rect2.area()
    );
}
