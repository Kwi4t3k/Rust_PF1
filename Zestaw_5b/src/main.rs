// Funkcja, która zwraca wartość cyfry rzymskiej jako u16 lub błąd w przypadku niepoprawnego znaku.
fn wartosc_cyfry_rzymskiej(c: char) -> Result<u16, String> {
    match c {
        'I' => Ok(1),    // 'I' oznacza 1
        'V' => Ok(5),    // 'V' oznacza 5
        'X' => Ok(10),   // 'X' oznacza 10
        'L' => Ok(50),   // 'L' oznacza 50
        'C' => Ok(100),  // 'C' oznacza 100
        'D' => Ok(500),  // 'D' oznacza 500
        'M' => Ok(1000), // 'M' oznacza 1000
        _ => Err("Znak nie jest cyfrą rzymską".to_string()), // Inny znak powoduje błąd
    }
}

// Funkcja, która konwertuje napis reprezentujący liczbę w systemie rzymskim na liczbę całkowitą.
// Zwraca wynik lub błąd w przypadku niepoprawnego napisu.
fn rzymskie(napis: &str) -> Result<u128, String> {
    let mut wynik = 0; // Zmienna do przechowywania wyniku końcowego
    let mut prev_value = 0; // Zmienna do przechowywania wartości poprzedniego znaku
    let mut count = 1; // Licznik powtórzeń aktualnej cyfry rzymskiej (np. III, IV, itp.)

    // Sprawdzamy, czy napis jest pusty
    if napis.is_empty() {
        return Err("Napis jest pusty".to_string()); // Jeśli napis jest pusty, zwracamy błąd
    }

    let mut prev_char = '\0'; // Zmienna do przechowywania poprzedniego znaku (zaczynamy od pustego znaku)

    // Iterujemy po wszystkich znakach w napisie od końca (od najmniej znaczącej cyfry)
    for c in napis.chars().rev() {
        let current_value = wartosc_cyfry_rzymskiej(c)?; // Sprawdzamy wartość cyfry rzymskiej

        // Sprawdzamy, czy nie ma powtórzeń w systemie rzymskim
        if c == prev_char {
            count += 1; // Jeśli cyfra się powtarza, zwiększamy licznik
        } else {
            prev_char = c; // Jeśli cyfra się zmienia, resetujemy licznik
            count = 1; // Rozpoczynamy nowe powtórzenie
        }

        // Jeśli cyfra powtarza się więcej razy niż dozwolone, zwracamy błąd
        match c {
            'I' if count > 3 => return Err("Niedozwolona ilość 'I'".to_string()), // I może występować max 3 razy
            'V' if count > 1 => return Err("Niedozwolona ilość 'V'".to_string()), // V może występować tylko raz
            'X' if count > 3 => return Err("Niedozwolona ilość 'X'".to_string()), // X może występować max 3 razy
            'L' if count > 1 => return Err("Niedozwolona ilość 'L'".to_string()), // L może występować tylko raz
            'C' if count > 3 => return Err("Niedozwolona ilość 'C'".to_string()), // C może występować max 3 razy
            'D' if count > 1 => return Err("Niedozwolona ilość 'D'".to_string()), // D może występować tylko raz
            'M' if count > 3 => return Err("Niedozwolona ilość 'M'".to_string()), // M może występować max 3 razy
            _ => (), // Brak specjalnej obsługi dla innych znaków (jeśli są prawidłowe)
        }

        // Sprawdzamy, czy bieżąca cyfra jest mniejsza od poprzedniej (oznacza to, że musimy odjąć jej wartość)
        if current_value < prev_value {
            wynik -= current_value as u128; // Odejmujemy wartość
        } else {
            wynik += current_value as u128; // Dodajemy wartość
        }

        prev_value = current_value; // Ustawiamy poprzednią wartość na bieżącą
    }

    // Zwracamy obliczoną liczbę
    Ok(wynik)
}

fn main() {
    // Testy poprawnych rzymskich liczb
    println!("{:?}", rzymskie("III"));  // Ok(3) - 3 jest reprezentowane jako III
    println!("{:?}", rzymskie("IX"));   // Ok(9) - IX to 9
    println!("{:?}", rzymskie("XIX"));  // Ok(19) - XIX to 19
    println!("{:?}", rzymskie("MCMX")); // Ok(1910) - MCMX to 1910

    // Testy błędów
    println!("{:?}", rzymskie("IJ"));   // Err("Znak nie jest cyfrą rzymską") - 'J' nie jest cyfrą rzymską
    println!("{:?}", rzymskie(""));     // Err("Napis jest pusty") - pusty napis
    println!("{:?}", rzymskie("IIII")); // Err("Niedozwolona ilość 'I'") - 'I' występuje 4 razy
    println!("{:?}", rzymskie("VV"));   // Err("Niedozwolona ilość 'V'") - 'V' występuje 2 razy
}