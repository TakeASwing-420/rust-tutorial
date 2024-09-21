#[allow(unused_variables)]
fn main() {
    let mut x = 10;
    // let r1 = &x; // immutable reference 1
    // let r2 = &x; // immutable reference 2
    let r3 = &mut x;

    println!("r3: {}",r3);
}
