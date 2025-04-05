fn odwroc_napis(napis: &str) -> String {
    napis.chars().rev().collect()
}

fn odwroc_napis(napis: &str) -> String {
    let mut result = String::new();

    for i in napis.chars().rev() {
        result.push_str(&i.to_string())
    }

    result
}

fn main() {
    println!("{}", odwroc_napis("Rust"));       // tsuR
    println!("{}", odwroc_napis("programowanie")); // einawomargorp
}