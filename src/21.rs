// Define a trait named `Summary` with a single method `summarize`.
trait Summary {
    fn summarize(&self) -> String;
}

// Implement the `Summary` trait for a `Book` struct.
struct Book {
    title: String,
    author: String,
}

impl Summary for Book {
    fn summarize(&self) -> String {
        format!("'{}' by {}", self.title, self.author)
    }
}

// Implement the `Summary` trait for an `Article` struct.
struct Article {
    headline: String,
    location: String,
}

impl Summary for Article {
    fn summarize(&self) -> String {
        format!("{} - {}", self.headline, self.location)
    }
}

fn main() {
    let book = Book {
        title: String::from("The Rust Programming Language"),
        author: String::from("Steve Klabnik and Carol Nichols"),
    };

    let article = Article {
        headline: String::from("Rust 2024 Roadmap Released"),
        location: String::from("rust-lang.org"),
    };

    println!("Book Summary: {}", book.summarize()); // Book Summary: 'The Rust Programming Language' by Steve Klabnik and Carol Nichols
    println!("Article Summary: {}", article.summarize()); // Article Summary: Rust 2024 Roadmap Released - rust-lang.org
}
