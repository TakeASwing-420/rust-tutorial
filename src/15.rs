#[derive(Debug)]
enum GameCommand {
    Move { x: i32, y: i32 },      // Movement with coordinates
    Attack(String),               // Attack a target (e.g., "dragon")
    PickItem(String),             // Pick an item (e.g., "sword")
    Quit,                         // Quit the game
}

impl GameCommand {
    fn execute(&self) {
        match self {
            GameCommand::Move { x: 0, y: 0 } => {
                println!("Moving to origin");
            }
            GameCommand::Move { x, y } => {
                println!("Moving to coordinates: ({}, {})", x, y);
            }
            GameCommand::Attack(target) => {
                println!("Attacking the {}", target);
            }
            GameCommand::PickItem(item) => {
                println!("Picking up a {}", item);
            }
            GameCommand::Quit => {
                println!("Quitting the game...");
            }
        }
    }
}

fn main() {
    let commands: Vec<GameCommand> = vec![
        GameCommand::Move { x: 0, y: 0 },
        GameCommand::Move { x: 10, y: 20 },
        GameCommand::PickItem(String::from("sword")),
        GameCommand::Attack(String::from("dragon")),
        GameCommand::Quit,
    ];

    for command in commands {
        command.execute();  // Execute each command
    }
    test();
}

// ---------------------------------------------------
#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
    // --snip--
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn test() {
    let coin = Coin::Penny;
    let another_coin = Coin::Quarter(UsState::Alaska);

    // Correct way to match and handle a `Coin::Quarter`
    if let Coin::Quarter(state) = another_coin {
        println!("State quarter from {state:?}!");

        // Now match the specific state
        match state {
            UsState::Alabama => println!("It's from Alabama"),
            UsState::Alaska => println!("It's from Alaska"),
        }
    }

    let mut count = 0;
    if let Coin::Quarter(state) = coin {
        println!("State quarter from {state:?}!");
    } else {
        count += 1;  // This block will execute because `coin` is a `Penny`
        println!("Coin is not a quarter, count: {}", count);
    }
}
