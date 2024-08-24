fn main() {
    let mut count = 0;
    let a = 'counting_up: loop {
        println!("count = {count}");
        let mut remaining: usize = 5 * 2;

        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break; // Simply break the inner loop
            }
            if count == 2 {
                break 'counting_up count * 2; // Break with a value, which will be assigned to `a`
            }
            remaining -= 1;
        }

        count += 1;
    };
    println!("End count = {count}, a = {a}"); // Print `a` value as well

    for_loop();
    println!(); // Print a newline after the for loop output
    println!("Here is the reverse of the number 23451: {}", while_loop(23451));
}

#[allow(unused)]
fn for_loop() {
    for i in 0..=10 {  // = is inclusive loop range
        print!("{i} ");
    }
}

#[allow(unused)]
fn while_loop(mut num: i128) -> i128 {
    let mut reversed: i128 = 0;
    while num != 0 {
        reversed = reversed * 10 + num % 10; //statement
        num /= 10;
    }
    reversed  //expression
}
