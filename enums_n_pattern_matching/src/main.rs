#[derive(Debug)] // I neeed to add this debug line above the enum in order to check it with {:?}!
enum IpAddrKind {
    V4(u8, u8, u8, u8), // enums can be defined with specific values, including structs & enums
    V6(String)
}

enum Message {
    Quit,
    Move {x: i32, y: i32},
    
}

#[derive(Debug)]
enum UsStates {
    Alabama,
    Alaska,
    // -- continued... 
}

#[derive(Debug)]
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsStates), // Quarters can be made with different states, so we use and enum of UsStates
}

// enum MyOption<T> { // The standard librarary makes Option<T> available by default so to avoid an
//     None, // error I need to rename this Option<T> so there are no naming conflicts.
//     Some(T),
// }

fn value_in_cents(coin: Coin) -> u8 {
    match coin { // The match expression checkd the incoming value against the possible enums.
        Coin::Penny => 1, // All enum possibilities must be handled.
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {:?}!", state); // And I can execute code before returning
            25
        }
    }
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None, // match must cover all variations or it will not compile.
        Some(i) => Some(i + 1),
    }
}

fn main() {
    let _four = IpAddrKind::V4;
    let home = IpAddrKind::V4(127, 0, 0, 1);
    let _loopback = IpAddrKind::V6(String::from("test"));
    
    route(home);
    
    let coin = Coin::Penny;
    let _nickel = Coin::Nickel;
    let _quarter = Coin::Quarter;
    let value = value_in_cents(coin);
    println!("The value of the coin is {}", value);

    // let number: i32 = 5;
    let anything = Some(5);
    let anything_plus_one = plus_one(anything);
    println!("{:?}", anything_plus_one);
    let none = plus_one(None);
    println!("{:?}", none);

    let dice_roll = 9;
    match dice_roll {
        9 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        // other => move_player(other) // Other is the default catchall and will use the value
        // passed
        // _ => reroll() // The _ character is similar to other but the value is not used. 
        _ => () // This simply means do nothing for other values.
    }
    fn add_fancy_hat() {}
    fn remove_fancy_hat() {}
    fn _move_player(_num_spaces: u8) {}
    fn _reroll() {}
    
    // Uting 'if let' is a way to avoid the boiler plate '_ => ()' syntax. 
    // The bullow code id functionally the same.
    // let mut count = 0;
    // match coin {
    //     Coin::Quarter(state) => println!("State quarter from {:?}!", state),
    //     _ => count += 1,
    // }
    let new_coin = Coin::Dime;
    let mut count = 0;
    if let Coin::Quarter(state) = new_coin {
        println!("State quarter from {:?}!", state);
    } else {
        count += 1;
    }

    println!("The count value is: {}", count);
}

fn route (ip: IpAddrKind) {
    println!("{:?}!", ip);
}
