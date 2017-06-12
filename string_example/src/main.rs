fn main() {
    let s = String::new();

    let data = "initial contents";
    let s = data.to_string();

    let s = "initial contents".to_string();

    println!("s's value is: {}", s);

    let mut s = String::from("foo");
    s.push_str("bar");

    println!("Now s's value is: {}", s);

    let mut s1 = String::from("foo");
    let s2 = String::from("bar");
    s1.push_str(&s2);
    s1.push('l');

    println!("s1's value is: {}", s1);
    println!("s2's value is: {}", s2);

    let s3 = String::from("Hello, ");
    let s4 = String::from("world");
    let s5 = s1 + &s2;

    println!("s5's value is: {}", s5);

    let s6 = String::from("tic");
    let s7 = String::from("tac");
    let s8 = String::from("toe");

    let s9 = format!("{}-{}-{}", s6, s7, s8);
    println!("s9's value is: {}", s9);

    for c in "你好世界!".chars() {
        println!("{}", c);
    }

}
