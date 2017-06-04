fn main() {
    let reference_to_nothing = dangle();

    println!("Thr value of dangle is {}",
        reference_to_nothing);
}

fn dangle() -> String {
    let s = String::from("hello");

    s
}