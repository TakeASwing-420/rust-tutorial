#[allow(unused_variables)]
fn main() {
    let mut x = 10;
    // let r1 = &x; // immutable reference 1
    // let r2 = &x; // immutable reference 2
    let r3 = &mut x;

    println!("r3: {}", r3);
    {
        let a = 10;
    }
    //  print!(a);

    let s = String::from("Hello World");
    let (s, res) = calculate_length_with_returning_ownership(s); // Getting back ownership
    println!("length of '{s}' is {res}");
    let res= calculate_length_with_reference(&s);
    println!("length of '{s}' is {res}");
}

fn calculate_length_with_returning_ownership(s: String) -> (String, usize) { //s has moved in the formal parameter
    let length = s.len();   // calculate the length 
    (s, length)             // return the string and its length as a tuple
}

fn calculate_length_with_reference(s: &String) -> usize {
    s.len()  // works with a reference, no need to return ownership
}
