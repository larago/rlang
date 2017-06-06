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

    let some_val = Some(3);
    match some_val {
        Some(3) => println!("three"),
        _ => (),
    }

    if let Some(3) = some_val {
        println!("three");
    }

    let mut count = 0;
    match coin {
        Coin::Quarter(state) => println!("State quarter from {:?}", state),
        _ => count + 1,
    }

    // if let Coin::Quarter(state) = coin {
    //     println!("State quarter from {:?}", state);
    // } else {
    //     count += 1;
    // }
}