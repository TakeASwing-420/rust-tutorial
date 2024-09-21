// Declare the module `two` and bring its content into scope
pub mod two;

fn main() {
    // Call the function from the `two` module
    two::main();
    
    let x = 5;
    let prompt = "The shadowing is over";
    let x = x + 1;  // Shadowing the previous value of x (5 + 1 = 6)

    {
        let x = x * 2; // Shadowing again in the inner scope (6 * 2 = 12)
        println!("The value of x in the inner scope is: {x}");
    }
    println!("The value of x is: {x}");  // Outer x is still 6
    println!("{prompt}");  // Prints: "The shadowing is over"
}
