use std::fs::File;
use std::io::{self, Read, ErrorKind};

fn main() {
    let greeting_file_result = File::open("hello.txt");

    let greeting_file = match greeting_file_result {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating the file: {e:?}"),
            },
            other_error => {
                panic!("Problem opening the file: {other_error:?}");
            }
        },
    };

    error_propagation();
}

// This function reads the contents of "hello.txt" and demonstrates error propagation
fn error_propagation() {
    match read_username_from_file() {
        Ok(content) => println!("File content: {}", content),
        Err(e) => eprintln!("Error occurred: {:?}", e),
    }
}

// Reads the content from "hello.txt"
fn read_username_from_file() -> Result<String, io::Error> {
    let mut username_file = File::open("hello.txt")?;
    let mut username = String::new();
    username_file.read_to_string(&mut username)?;
    Ok(username)
}
