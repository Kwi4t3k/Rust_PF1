use std::ops::Add;

fn szyfruj(napis: &str, klucz: u32) -> String {
    let mut wynik = String::new();

    for i in napis.chars().rev() {
        wynik.push(i);
    }

    wynik
}

fn main() {
    println!("{}", szyfruj("Aladyn", 2))
}
