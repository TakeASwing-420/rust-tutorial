fn main() {
    let width1 = 30;
    let height1 = 50;
    let dims = (width1, height1); // Both are u32

    println!(
        "The area of the rectangle is {} square pixels.",
        area(dims)
    );
}

fn area(dimensions: (u32, u32)) -> u32 {
    return (dimensions.0) * (dimensions.1) as u32; // Convert both to u64
}
