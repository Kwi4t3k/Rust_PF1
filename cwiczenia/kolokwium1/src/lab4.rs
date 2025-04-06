mod lab4;

fn liczba_wystapien(napis: String, znak: char) -> usize {
    let mut licznik = 0;

    for c in napis.chars() {
        if c == znak {
            licznik += 1;
        }
    }

    licznik
}

fn liczba_wystapien_iterator(napis: String, znak: char) -> usize {
    napis.chars().filter(|x| *x == znak).count()
}

fn rzymskie(napis: &str) -> i32 {
    let mut wynik = 0;
    let mut poprzednia = 0;

    for znak in napis.chars().rev() {
        let wartosc_znaku = match znak {
            'I' => 1,
            'V' => 5,
            'X' => 10,
            'L' => 50,
            'C' => 100,
            'D' => 500,
            'M' => 1000,
            _ => 0,
        };

        if wartosc_znaku < poprzednia {
            wynik -= wartosc_znaku;
        } else {
            wynik += wartosc_znaku;
        }

        poprzednia = wartosc_znaku;
    }

    wynik
}

fn main() {
    println!("{}", liczba_wystapien(String::from("ananas"), 'a')); // 3 - a
    println!("{}", liczba_wystapien_iterator(String::from("ananas"), 'a')); // 3 - a

    println!("-------------------");

    println!("{}", rzymskie("III")); // 3
    println!("{}", rzymskie("IX")); // 9
    println!("{}", rzymskie("XIX")); // 19
    println!("{}", rzymskie("MCMX")); // 1910

    println!("-------------------");
}