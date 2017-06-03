fn main() {
    let number = 7;

    if number < 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }

    let number2 = 3;

    if number2 != 0 {
        println!("number was something other than zero.");
    }

    if number % 4 == 0 {
        println!("number is divisiable by 4");
    } else if number % 3 == 0 {
        println!("number is divisiable by 3");
    } else if number % 2 == 0 {
        println!("number is dicisiable by 2");
    } else {
        println!("number is not divisiable by 4, 3 or 2");
    }

    // let and if
    let condition = true;
    let number = if condition {
        5
    } else {
        6
        //"six"
    };

    println!("The value of number is: {}", number);
}