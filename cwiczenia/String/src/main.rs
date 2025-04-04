use std::fmt::format;

fn main() {
    let s = "fsafgasf".to_string();

    let s = String::from("dsadsafa");

    let mut s = String::from("dupa");
    s.push_str(" rzepa"); // string slice (&str)
    s.push('!');             // char / jeden znak

    println!("{s}");

    // łączenie
    let s1 = String::from("dupa "); // s1 już nie istnieje po zrobieniu s3
    let s2 = String::from("rzepa");
    let s3 = s1 + &s2; // zabiera s1 ownership | nie można dodawać dwóch zwykłych wartości więc jedna ma referencję
    println!("{s3}");

    // formatowanie stringów
    let good = String::from("Good");
    let morning = String::from("dzień");

    let full_message = format!("{good} {morning}");
    println!("{full_message}");
}