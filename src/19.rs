use std::fmt::Display;

struct Point<T> {
    x: T,
    y: T,
}

// Adding `Display` constraint to `T`
impl<T: Display> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }

    // Implement a method using a new generic type `U` for the `Point<T>` struct.
    fn message<U>(&self, g: U) -> String
    where
        U: Display, // Ensure `U` can be printed with `format!`
    {
        format!("{g} {x}, {y}", g = g, x = &self.x, y = &self.y)
    }
}

fn main() {
    let p = Point { x: 5, y: 10 };

    println!("p.x = {}", p.x());
    println!("Message: {}", p.message("Coordinates: "));
}
