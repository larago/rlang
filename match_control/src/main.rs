enum Coin {
    Penny,
    Nickle,
    Dime,
    Quarter(UsState),
}

#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
}

fn value_in_cents(coin: Coin) -> i32 {
    match coin {
        Coin::Penny => {
            println!("Lucky penny!");
            1
        },
        Coin::Nickle => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {:?}", state);
            25
        },
    }
}

fn main() {
    value_in_cents(Coin::Quarter(UsState::Alaska));

    let some_u8_value = 7;
    match some_u8_value {
        1 => println!("one"),
        2 => println!("three"),
        3 => println!("five"),
        7 => println!("seven"),
        _ => (),
    }
}