fn dodaj_pisemnie(a: &str, b: &str) -> String {
    let mut wynik = String::new(); // Tutaj będziemy budować wynik dodawania jako napis
    let mut przeniesienie = 0;     // Zmienna do przechowywania przeniesienia (np. 1 gdy suma > 9)

    // Zamieniamy napisy na wektory znaków, żeby łatwo móc odwoływać się do każdej cyfry
    let a_chars: Vec<char> = a.chars().collect();
    let b_chars: Vec<char> = b.chars().collect();

    // Pobieramy indeksy ostatnich cyfr (bo dodajemy od końca jak pisemnie)
    let mut i = a_chars.len() as i32 - 1;
    let mut j = b_chars.len() as i32 - 1;

    // Pętla trwa dopóki są cyfry do dodania lub jest przeniesienie
    while i >= 0 || j >= 0 || przeniesienie > 0 {
        let mut suma = przeniesienie; // Zaczynamy od przeniesienia (na początku 0)

        // Jeśli są jeszcze cyfry w liczbie a, dodajemy je do sumy
        if i >= 0 {
            // Zamieniamy znak na cyfrę
            suma += a_chars[i as usize].to_digit(10).unwrap();
            i -= 1; // Przechodzimy do poprzedniej cyfry
        }

        // Jeśli są jeszcze cyfry w liczbie b, dodajemy je do sumy
        if j >= 0 {
            suma += b_chars[j as usize].to_digit(10).unwrap();
            j -= 1; // Przechodzimy do poprzedniej cyfry
        }

        // Obliczamy nowe przeniesienie (np. 15 -> przeniesienie 1)
        przeniesienie = suma / 10;

        // Dodajemy ostatnią cyfrę do wyniku (np. z 15 bierzemy 5)
        let cyfra = (suma % 10).to_string();
        wynik = cyfra + &wynik; // Dodajemy cyfrę na początek wyniku
    }

    wynik // Zwracamy końcowy napis z wynikiem
}

fn main() {
    println!("{}", dodaj_pisemnie("1", "3")); // 4
    println!("{}", dodaj_pisemnie("8", "3")); // 11
    println!("{}", dodaj_pisemnie("10", "23")); // 33
    println!("{}", dodaj_pisemnie("1", "0")); // 1
    println!("{}", dodaj_pisemnie("11", "00")); // 11
    println!("{}", dodaj_pisemnie("131", "9900")); // 10031
    println!("{}", dodaj_pisemnie("998", "7")); // 1005
    println!("{}", dodaj_pisemnie("24872947", "294729478")); // 319602425
    println!("{}", dodaj_pisemnie("5924729874298749827418582", "6782893629472094209740298")); // 12707623503770844037158880
}