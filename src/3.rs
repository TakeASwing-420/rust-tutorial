use rand::Rng; // Importing the 'Rng' trait which defines random number generation functions
use std::cmp::Ordering; // Importing 'Ordering' for comparison results (Less, Greater, Equal)
use std::io; // Importing 'io' for input/output handling

// This is the main function where the game logic is implemented
fn main() {
    println!("Guess the number!"); // Prints an introductory message to the user

    // Generate a random number between 1 and 100 (inclusive)
    let secret_number = rand::thread_rng().gen_range(1..=100);

    // Start an infinite loop for the game to repeatedly ask for input until the user guesses correctly
    loop {
        println!("Please input your guess."); // Prompt the user to enter their guess

        let mut guess = String::new(); // Create a mutable variable 'guess' to hold user input, initialized as an empty string

        // Read the user input from standard input (stdin) and store it in 'guess'
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line"); // If reading fails, this message will be shown

        // Convert the input string into a number (u32). If conversion fails (due to invalid input), continue to the next iteration
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num, // If input is a valid number, it will be stored in 'guess'
            Err(_) => continue, // If parsing fails, the loop will restart (ignores invalid input)
        };

        // Print the guess that was just entered by the user
        println!("You guessed: {guess}");

        // Compare the user's guess with the secret number
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"), // If guess is smaller, print "Too small!"
            Ordering::Greater => println!("Too big!"), // If guess is larger, print "Too big!"
            Ordering::Equal => { // If guess matches the secret number
                println!("You win!"); // Print "You win!" and break out of the loop
                break;
            }
        }
    }
}
