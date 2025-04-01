fn main() {
    // 1. Each value in Rust has a variable that's its owner

    let s1 = String::from("RUST");
    let len = calculate_lenght(&s1);
    println!("Length of {} is {}", s1, len);

    // 2. There can only be one owner at a time
    let s2 = s1;
    // println!("{}", s1); ❌
    println!("{}", s2); // ✔️

    // 3. When the owner goes out of scope, the value will be dropped
    let s1 = String::from("RUST");
    let len = calculate_lenght(&s1);
    println!("Lenght of {} is {}", s1, len);
}
// s1 goes out of scope and its value will be dropped
fn print_lost(s: &String) {
    println!("{}", &s1);  // cannot find value `s1` in this scope
}

fn calculate_lenght(s: &String) -> usize {
    s.len()
}