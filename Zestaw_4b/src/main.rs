fn dodaj_pismnie(napis1: &str, napis2: &str) -> String {
    let mut wynik = String::new();
    let mut pozyczka = 0;

    let liczby1: Vec<char> = napis1.chars().collect();
    let liczby2: Vec<char> = napis2.chars().collect();

    let mut max_dlugosc;
    if liczby1.len() >= liczby2.len() {
        max_dlugosc = liczby1.len();
    } else {
        max_dlugosc = liczby2.len();
    }

    for i in 0..max_dlugosc {
        let cyfra1
    }

    wynik
}

fn main() {
    println!("{}", dodaj_pismnie("1", "3")); // 4
    println!("{}", dodaj_pismnie("8", "3")); // 11
    println!("{}", dodaj_pismnie("10", "23")); // 33
    println!("{}", dodaj_pismnie("1", "0")); // 1
    println!("{}", dodaj_pismnie("11", "00")); // 11
    println!("{}", dodaj_pismnie("131", "9900")); // 10031
    println!("{}", dodaj_pismnie("998", "7")); // 1005
    println!("{}", dodaj_pismnie("24872947", "294729478")); // 319602425
    println!("{}", dodaj_pismnie("5924729874298749827418582", "6782893629472094209740298")); // 12707623503770844037158880
}