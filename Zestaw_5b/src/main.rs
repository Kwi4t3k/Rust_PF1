fn dodaj_pisemnie(a: &str, b: &str) -> Result<String, String> {
    let mut wynik = String::new();
    let mut przeniesienie = 0;

    let a_chars: Vec<char> = a.chars().collect();
    let b_chars: Vec<char> = b.chars().collect();

    let mut i = a_chars.len() as i32 - 1;
    let mut j = b_chars.len() as i32 - 1;

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

}