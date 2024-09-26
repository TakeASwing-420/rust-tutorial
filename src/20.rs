fn main() {
    // Define a closure that captures a value from its environment.
    let x = 10;
    let add_x = |num: i32| num + x;

    println!("Adding 5 to x (10): {}", add_x(5)); // Output: 15

    // Example of a closure that modifies the captured environment.
    let mut count = 0;
    let mut increment = || {
        count += 1;
        println!("Count: {}", count);
    };

    increment(); // Count: 1
    increment(); // Count: 2

    // Using closures as function parameters
    let nums = vec![1, 2, 3, 4, 5];
    let sum: i32 = nums.iter().map(|n| n + x).sum();
    println!("Sum of elements + x: {}", sum); // Sum of elements + x: 65
}
