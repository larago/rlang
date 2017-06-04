fn main() {
    let mut s1 = String::from("hello");

    let len = calcaulate_length(&s1);

    println!("The length of {} is {}", s1, len);

    change(&mut s1);
}

fn calcaulate_length(s: &String) -> usize {
    s.len()
}

fn change(somr_string: &mut String) {
    some_string.push_str(" world!");
}