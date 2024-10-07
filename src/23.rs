use std::fmt::Display;

trait DisplayInfo {
    fn display(&self) -> String; // Changed return type to String
}

#[derive(Clone)] // Implementing Clone for User
struct User<'a> {
    info: &'a str,
}

impl<'a> DisplayInfo for User<'a> {
    fn display(&self) -> String {
        self.info.to_string() // Convert &str to String
    }
}

#[derive(Clone)] // Implementing Clone for Robot
struct Robot<'a> {
    id: u32,
    info: &'a str,
}

impl<'a> DisplayInfo for Robot<'a> {
    fn display(&self) -> String {
        format!("Robot ID: {}, Info: {}", self.id, self.info)
    }
}

fn static_display<'daniel, T: DisplayInfo>(item: &'daniel T, message: &'daniel str) -> String {
    println!("{}",message);
    item.display()
}

// `take_ownership` works for types implementing `Display`.
fn take_ownership<T: Display>(item: T) {
    println!("Took ownership of {}", item);
}

fn main() {
    let user1 = User { info: "User data 1" };
    let user2 = User { info: "User data 2" };
    let robot1 = Robot { id: 1, info: "Robot data 1" };
    let robot2 = Robot { id: 2, info: "Robot data 2" };
    let exper = String::from("Robot data 3");
    let exp_id = 3;
    let robot3 = Robot { id: exp_id, info: exper.as_str() };

    // Display the third robot's data
    println!("{}", robot3.display());
    take_ownership(exp_id);
    println!("{}", robot3.display());
    
    println!("{}",static_display(&user1, "hello I am daniel"));

    // Create a vector of trait objects (dynamic dispatch)
    let referenced_dispatched_displayables: Vec<&dyn DisplayInfo> = vec![&user1, &user2, &robot1, &robot2];
    
    // Use cloning to avoid move issues
    let mut owned_dynamic_displayables: Vec<Box<dyn DisplayInfo>> = Vec::new();
    owned_dynamic_displayables.push(Box::new(user2.clone())); // Clone user2
    owned_dynamic_displayables.push(Box::new(user1.clone())); // Clone user1
    owned_dynamic_displayables.push(Box::new(robot1.clone())); // Move robot1 (not borrowed previously)
    owned_dynamic_displayables.push(Box::new(robot2.clone())); // Move robot2 (not borrowed previously)

    // Display information for each trait object
    for (owned, referenced) in owned_dynamic_displayables.iter().zip(referenced_dispatched_displayables.iter()) {
        println!("Owned: {}, Referenced: {}", owned.display(), referenced.display());
    }
}
