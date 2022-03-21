#[derive(Debug)]
struct Rectangle {
    width : u32,
    height : u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width*self.height
    }

    fn square(size: u32) -> Rectangle {
        Rectangle {
            width : size,
            height : size,
        }
    }
}


fn main() {

    let rect1 = Rectangle::square(40);

    println!(
        "The area of the rectanghle is {} square pixels.",
        rect1.area()
    );
}