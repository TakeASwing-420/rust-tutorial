fn main(){
    #[derive(Debug)]
    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];

    for i in &row{
        println!("{i:?}");
    }


    /*let mut vola = vec![1, 2, 3, 4, 5];

    let first = &vola[0];

    vola.push(6);


    println!("The first element is: {first}");
    */
      //Attempting to add an element to a vector while holding a reference to an item
}