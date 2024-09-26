use std::fmt::Display; 

trait DisplayInfo {
    fn display(&self) -> String; // Change return type to String
}

struct User<'a> {
    info: &'a str,
}

impl<'a> DisplayInfo for User<'a> {
    fn display(&self) -> String { // Change return type to String
        self.info.to_string() // Convert to String
    }
}

struct Robot<'a> {
    id: u32,
    info: &'a str,
}

impl<'a> DisplayInfo for Robot<'a> {
    fn display(&self) -> String { // Change return type to String
        format!("Robot ID: {}, Info: {}", self.id, self.info) // Return a String
    }
}

fn take_ownership<T: Display>(item: T) {
    println!("Took ownership of {}", item); // Displaying using the Display trait
}

fn main() {
    let user1 = User { info: "User data 1" };
    let user2 = User { info: "User data 2" };
    let robot1 = Robot { id: 1, info: "Robot data 1" };
    let robot2 = Robot { id: 2, info: "Robot data 2" };
    let exper =  String::from("Robot data 3");
    let exp_id = 3;
    let robot3 = Robot { id: exp_id, info: exper.as_str()};
    println!("{}", robot3.display());
    take_ownership(exp_id);
    println!("{}", robot3.display());
    // take_ownership(exper);                      //Commenting out this line will cause a compile-time error              
    // println!("{}", robot3.display());   

    // Create a vector of trait objects
    let mut displayables: Vec<Box<&dyn DisplayInfo>> = Vec::new();

    // Add User and Robot instances to the vector
    displayables.push(Box::new(&user1));
    displayables.push(Box::new(&user2));
    displayables.push(Box::new(&robot1));
    displayables.push(Box::new(&robot2));

    // Display information for each trait object
    for displayable in displayables {
        println!("{}", displayable.display());
    }
}
