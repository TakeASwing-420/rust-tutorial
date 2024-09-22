#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u64 {
        // Cast both width and height to u64 before multiplication
        (self.width as u64) * (self.height as u64)
    }
}

impl Rectangle {
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

fn main() {
    let r1 = Rectangle {
        width: 700,
        height: 800,
    };
    println!("The rectangle is {r1:#?}");
    println!("Area is {} units", r1.area());
}
