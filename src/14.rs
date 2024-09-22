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

    fn square(size: u32)->Self{
        Self { width: size, height: size }
    }
}

impl Rectangle {
    fn can_hold(&self, other: &Rectangle, message: &str) -> bool {
        print!("{message}");
        self.width > other.width && self.height > other.height
    }
}

fn main() {
    let r1 = Rectangle {
        width: 700,
        height: 800,
    };

    let s = Rectangle::square(478);
    println!("The rectangle is {r1:#?}");
    println!("Area is {} units", r1.area());
    println!("{}",r1.can_hold(&s, "HEllo GuyZ"));
    println!("Area is {:?} units", s);

}
