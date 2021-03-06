#[derive(Debug)] // so we can inspect the state in a minute
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

fn main() {
    println!("Penny: {:?}", value_in_cents(Coin::Penny));
    println!("Nickel: {:?}", value_in_cents(Coin::Nickel));
    println!("Dime: {:?}", value_in_cents(Coin::Dime));
    println!("Quarter: {:?}", value_in_cents(Coin::Quarter(UsState::Alaska)));
    println!("Quarter: {:?}", value_in_cents(Coin::Quarter(UsState::Alabama)));

    match_placeholder();
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {:?}!", state);
            25
        }
    }
}

fn match_placeholder() {
    let some_u8_value = 0u8;
    match some_u8_value {
        1 => println!("one"),
        3 => println!("three"),
        5 => println!("five"),
        7 => println!("seven"),
        _ => (),
    }
}
