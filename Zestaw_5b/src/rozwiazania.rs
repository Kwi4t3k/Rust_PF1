1. Napisz funkcję
fn wartosc_cyfry(c: char) -> Result<u8, String>
która zwraca wartość cyfry dziesiętnej podanej jako znak — albo opis tekstowy błędu, jesli znak nie jest cyfrą.


fn wartosc_cyfry(c: char) -> Result<u8, String> {
    if !c.is_digit(10) {
        Err("znak nie jest cyfrą".to_string())
    } else {
        Ok(c.to_digit(10).unwrap() as u8)
    }
}


fn wartosc_cyfry(c: char) -> Result<u8, String> {
    if !c.is_digit(10) {
        return Err("znak nie jest cyfrą".to_string())
    }

    // `to_digit(10)` próbuje przekonwertować znak na liczbę dziesiętną
    // Zwróci `Some(u32)` jeśli znak jest cyfrą (np. '5' => Some(5)) lub `None` jeśli nie jest cyfrą
    // Używamy `ok_or_else` do zamiany `None` na `Err` z opisem błędu
    // Jeśli `to_digit(10)` zwróci `None`, `?` propaguje błąd
    let cyfra = c.to_digit(10).ok_or_else(|| "Błąd kompresji".to_string())? as u8;

    Ok(cyfra)
}

fn main() {
    println!("{:?}", wartosc_cyfry('5'));
    println!("{:?}", wartosc_cyfry('9'));
    println!("{:?}", wartosc_cyfry('a'));
    println!("{:?}", wartosc_cyfry('/'));
}


2. Napisz funkcję o nagłówku
fn dodaj_pisemnie(a: &str, b: &str) -> Result<String, String>
która doda dwie liczby naturalne podane w argumentach jako napisy w zapisie dziesiętnym (niekoniecznie poprawne; puste napisy także uznajemy za niepoprawne) — i zwróci wynik również jako napis (lub napisowy opis błędu). Uwaga: dodawanie należy przeprowadzić pisemnie, bowiem liczby mogą być dowolnie duże.
Uwaga! Użyj funkcji z poprzedniego zadania i operatora ?


fn dodaj_pisemnie(a: &str, b: &str) -> Result<String, String> {
    // Sprawdzamy, czy któryś z napisów jest pusty, jeśli tak, zwracamy błąd
    if a.is_empty() || b.is_empty() {
        return Err("Nieprawidłowy argument: jeden z napisów jest pusty".to_string());
    }

    // Sprawdzamy, czy oba napisy zawierają tylko cyfry
    if !a.chars().all(|c| c.is_digit(10)) || !b.chars().all(|c| c.is_digit(10)) {
        return Err("Nieprawidłowy argument: napis zawiera niedozwolone znaki".to_string());
    }

    let mut wynik = String::new(); // String, w którym będziemy przechowywać wynik dodawania
    let mut przeniesienie = 0;     // Zmienna do przechowywania przeniesienia (np. 1, jeśli suma > 9)

    // Zamieniamy napisy na wektory znaków, żeby łatwo odwoływać się do poszczególnych cyfr
    let a_chars: Vec<char> = a.chars().collect();
    let b_chars: Vec<char> = b.chars().collect();

    // Indeksy do iterowania po znakach od końca napisu (od najmniej znaczącej cyfry)
    let mut i = a_chars.len() as i32 - 1;
    let mut j = b_chars.len() as i32 - 1;

    // Pętla trwa dopóki są cyfry do dodania lub jest przeniesienie
    while i >= 0 || j >= 0 || przeniesienie > 0 {
        let mut suma = przeniesienie; // Zaczynamy od przeniesienia

        // Jeśli są jeszcze cyfry w pierwszym napisie, dodajemy je do sumy
        if i >= 0 {
            // Przekonwertowujemy znak na cyfrę i dodajemy ją do sumy
            suma += a_chars[i as usize].to_digit(10).ok_or_else(|| "Błąd: niepoprawna cyfra w pierwszym napisie".to_string())?;
            i -= 1; // Przechodzimy do poprzedniej cyfry
        }

        // Jeśli są jeszcze cyfry w drugim napisie, dodajemy je do sumy
        if j >= 0 {
            suma += b_chars[j as usize].to_digit(10).ok_or_else(|| "Błąd: niepoprawna cyfra w drugim napisie".to_string())?;
            j -= 1; // Przechodzimy do poprzedniej cyfry
        }

        // Obliczamy nowe przeniesienie (np. jeśli suma wynosi 15, to przeniesienie = 1)
        przeniesienie = suma / 10;

        // Dodajemy ostatnią cyfrę do wyniku (np. z 15 bierzemy 5)
        let cyfra = (suma % 10).to_string();
        wynik = cyfra + &wynik; // Dodajemy cyfrę na początek wyniku
    }

    // Zwracamy wynik w postaci String
    Ok(wynik)
}

fn main() {
    // Przykłady wywołań funkcji dodaj_pisemnie i wypisanie wyników
    println!("{}", dodaj_pisemnie("1", "3").unwrap()); // 4
    println!("{}", dodaj_pisemnie("8", "3").unwrap()); // 11
    println!("{}", dodaj_pisemnie("10", "23").unwrap()); // 33
    println!("{}", dodaj_pisemnie("1", "0").unwrap()); // 1
    println!("{}", dodaj_pisemnie("11", "00").unwrap()); // 11
    println!("{}", dodaj_pisemnie("131", "9900").unwrap()); // 10031
    println!("{}", dodaj_pisemnie("998", "7").unwrap()); // 1005
    println!("{}", dodaj_pisemnie("24872947", "294729478").unwrap()); // 319602425
    println!("{}", dodaj_pisemnie("5924729874298749827418582", "6782893629472094209740298").unwrap()); // 12707623503770844037158880

    // Przykłady błędów
    println!("{:?}", dodaj_pisemnie("abc", "123")); // Błąd (niepoprawne znaki)
    println!("{:?}", dodaj_pisemnie("", "123"));    // Błąd (pusty napis)
}


3. Napisz funkcję
fn wartosc_cyfry_rzymskiej(c: char) -> Result<u16, String>
która zwraca wartość cyfry rzymskiej podanej jako znak — albo opis tekstowy błędu, jesli znak nie jest cyfrą rzymską (jednym z: I V X L C D M).


fn wartosc_cyfry_rzymskiej(c: char) -> Result<u16, String> {
    if c == 'I' {
        Ok(1)
    } else if c == 'V' {
        Ok(5)
    } else if c == 'X' {
        Ok(10)
    } else if c == 'L' {
        Ok(50)
    } else if c == 'C' {
        Ok(100)
    } else if c == 'D' {
        Ok(500)
    } else if c == 'M' {
        Ok(1000)
    } else {
        return Err("Znak nie jest cyfrą rzymską".to_string());
    }
}

fn wartosc_cyfry_rzymskiej(c: char) -> Result<u16, String> {
    match c {
        'I' => Ok(1),
        'V' => Ok(5),
        'X' => Ok(10),
        'L' => Ok(50),
        'C' => Ok(100),
        'D' => Ok(500),
        'M' => Ok(1000),
        _ => Err("Znak nie jest cyfrą rzymską".to_string()),
    }
}

fn main() {
    println!("{:?}", wartosc_cyfry_rzymskiej('I')); // Ok(1)
    println!("{:?}", wartosc_cyfry_rzymskiej('V')); // Ok(5)
    println!("{:?}", wartosc_cyfry_rzymskiej('X')); // Ok(10)
    println!("{:?}", wartosc_cyfry_rzymskiej('Z')); // Err("Znak nie jest cyfrą rzymską")
}


4. Napisz funkcję o nagłówku
fn rzymskie(napis: &str) -> Result<u128, String>
która dla napisu reprezentującego liczbę w zapisie rzymskim (nie zakładamy jego poprawności; ponadto pusty ciąg także jest nie poprawny) zwraca liczbę reprezentowaną przez ów napis lub napisowy opis błędu. Błędy mogą być trojakie — niewłaściwa cyfra (o tym powiadomi nas poniższa funkcja pomocnicza); pusty napis; niewłaściwa kolejność cyfr.
Uwaga! Użyj funkcji z poprzedniego zadania i operatora ?

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