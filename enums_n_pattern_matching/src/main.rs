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

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsStates), // Quarters can be made with different states, so we use and enum of UsStates
}

enum Option<T> {
    None,
    Some(T),
}

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

fn plus_one(x: Option<i32>) -> i32 {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}

fn main() {
    let _four = IpAddrKind::V4;
    let home = IpAddrKind::V4(127, 0, 0, 1);
    let _loopback = IpAddrKind::V6(String::from("test"));

    route(home);
    
    let coin = Coin::Penny;
    let value = value_in_cents(coin);
    println!("The value of the coin is {}", value);
    
    let anything = Some(5);
    let anything_plus_one = plus_one(anything);
    println!("{}", anything_plus_one)
}

fn route (ip: IpAddrKind) {
    println!("{:?}!", ip);
}
