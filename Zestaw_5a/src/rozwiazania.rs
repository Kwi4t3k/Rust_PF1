1. Napisz funkcję o nagłówku
fn zamien_syst8_na_syst2(z: &str) -> Option<String>
zamieniającą zapis liczby całkowitej bez znaku w systemie ósemkowym na zapis w systemie dwójkowym. Wynik ma być najkrótszy możliwy, niepusty. Wynik None ma oznaczać wystąpienie w parametrze z niedozwolonego znaku (spoza cyfr ósemkowych) lub pusty napis w parametrze.


fn zamien_syst8_na_syst2(z: &str) -> Option<String> {
    // Sprawdzamy, czy napis jest pusty
    if z.is_empty() {
        return None; // Zwracamy None, jeśli napis jest pusty
    }

    // Sprawdzamy, czy w napisie są tylko dozwolone cyfry ósemkowe (0-7)
    for c in z.chars() {
        if !c.is_digit(8) { // Digit(8) oznacza cyfrę w systemie ósemkowym
            return None; // Jeśli znajdziemy niedozwolony znak, zwracamy None
        }
    }

    // Konwertujemy napis ósemkowy na liczbę (u32), a potem na system dwójkowy
    let liczba = u32::from_str_radix(z, 8).unwrap(); // Konwertujemy napis z systemu ósemkowego do liczby dziesiętnej
    Some(format!("{:b}", liczba)) // {:b} formatuje na system binarny
}

fn zamien_syst8_na_syst2(z: &str) -> Option<String> {
    // Sprawdzamy, czy napis jest pusty
    if z.is_empty() {
        return None; // Jeśli napis jest pusty, zwracamy None
    }

    // Sprawdzamy, czy w napisie są tylko dozwolone cyfry ósemkowe (0-7)
    for c in z.chars() {
        if !c.is_digit(8) { // Sprawdzamy, czy każda cyfra należy do systemu ósemkowego
            return None; // Zwracamy None, jeśli znajdziemy niedozwolony znak
        }
    }

    // Konwertowanie z ósemkowego na dziesiętny
    let mut liczba_dziesietna = 0; // Zmienna do przechowywania liczby w systemie dziesiętnym
    let mut potega = 1; // Potęga 8, zaczynając od najmniej znaczącej cyfry

    // Przechodzimy przez każdą cyfrę napisu w odwrotnej kolejności
    for c in z.chars().rev() { // Odwracamy napis, żeby pracować od najmniej znaczącej cyfry
        let cyfra = c.to_digit(10).unwrap(); // Zamieniamy znak na cyfrę dziesiętną
        liczba_dziesietna += cyfra * potega; // Dodajemy cyfrę pomnożoną przez odpowiednią potęgę ósemki
        potega *= 8; // Zwiększamy potęgę ósemki (8^0, 8^1, 8^2, ...)
    }

    // Konwertowanie z dziesiętnego na binarny
    let mut wynik_binarny = String::new(); // Zmienna do przechowywania wyniku w systemie binarnym
    if liczba_dziesietna == 0 {
        return Some("0".to_string()); // Obsługujemy specjalny przypadek dla liczby 0 (jej zapis binarny to "0")
    }

    // Dzielimy liczbę przez 2 i zapisujemy reszty, aż liczba osiągnie 0
    while liczba_dziesietna > 0 {
        wynik_binarny.push_str(&(liczba_dziesietna % 2).to_string()); // Dodajemy resztę z dzielenia przez 2 (bit)
        liczba_dziesietna /= 2; // Dzielimy liczbę przez 2
    }

    // Odwracamy wynik, ponieważ konwertowaliśmy od najmniej znaczącego bitu
    Some(wynik_binarny.chars().rev().collect()) // Zwracamy wynik w postaci napisu, odwróconego
}

fn main() {
    println!("{:?}", zamien_syst8_na_syst2("10"));  // Some("1000")
    println!("{:?}", zamien_syst8_na_syst2("7"));   // Some("111")
    println!("{:?}", zamien_syst8_na_syst2("0"));   // Some("0")
    println!("{:?}", zamien_syst8_na_syst2("123")); // Some("1010011")
    println!("{:?}", zamien_syst8_na_syst2("8"));   // None (niepoprawna cyfra)
    println!("{:?}", zamien_syst8_na_syst2(""));    // None (pusty napis)
}


2. Napisz funkcję o nagłówku
fn wartosc_syst2(z: &str) -> Option<u8>
obliczającą wartość całkowitą bez znaku zapisaną w systemie dwójkowym — pod warunkiem, że mieści się na ośmiu bitach. Jeśli nie (lub w zapisie występuje znak inny niż cyfra dwójkowa lub parametr jest pusty), to wynikiem jest None.


fn wartosc_syst2(z: &str) -> Option<u8> {
    // Sprawdzamy, czy napis jest pusty
    if z.is_empty() {
        return None; // Jeśli napis jest pusty, zwracamy None
    }

    // Sprawdzamy, czy w napisie są tylko cyfry '0' i '1'
    // `z.chars().all(...)` sprawdza, czy wszystkie znaki w napisie są '0' lub '1'
    if !z.chars().all(|c| c == '0' || c == '1') {
        return None; // Jeśli w napisie są znaki inne niż '0' lub '1', zwracamy None
    }

    // Sprawdzamy, czy długość napisu mieści się w 8 bitach (czyli w zakresie 0..=255)
    if z.len() > 8 {
        return None; // Jeśli długość napisu jest większa niż 8, zwracamy None, bo nie mieści się w 8 bitach
    }

    // Konwertujemy napis binarny na liczbę dziesiętną
    let mut wynik = 0; // Zmienna do przechowywania wyniku w systemie dziesiętnym
    // `enumerate()` daje nam indeks i wartość w każdej iteracji (indeks to potęga liczby 2)
    for (i, c) in z.chars().rev().enumerate() {
        // Jeśli znak jest '1', dodajemy do wyniku odpowiednią potęgę 2 (2^i)
        if c == '1' {
            wynik += 2u8.pow(i as u32); // Potęgujemy 2 do potęgi `i` i dodajemy do wyniku
        }
    }

    // Sprawdzamy, czy wynik mieści się w zakresie 0..=255
    if wynik <= u8::MAX {
        Some(wynik) // Jeśli wynik mieści się w zakresie, zwracamy go w postaci `Some(wynik)`
    } else {
        None // Jeśli wynik przekracza zakres, zwracamy None
    }
}

fn main() {
    println!("{:?}", wartosc_syst2("110"));      // Some(6)
    println!("{:?}", wartosc_syst2("101010"));   // Some(42)
    println!("{:?}", wartosc_syst2("100000000")); // None (przekracza 8 bitów)
    println!("{:?}", wartosc_syst2("abc"));      // None (niepoprawne znaki)
    println!("{:?}", wartosc_syst2("10110010"));  // Some(178)
}


3. Napisz funkcję o nagłówku
fn wartosc_syst8(z: &str) -> Option<u8>
obliczającą wartość całkowitą bez znaku zapisaną w systemie ósemkowym — pod warunkiem, że mieści się na ośmiu bitach. Jeśli nie (lub w zapisie występuje znak inny niż cyfra ósemkowa lub parametr jest pusty), to wynikiem jest None.
Uwaga 1: Funkcję tę należy zbudować z funkcji z zadań poprzednich.
Uwaga 2: Zrób dwie wersje tej funkcji — pierwszą bez znaku zapytania, a drugą ze znakiem zapytania.


fn wartosc_syst8(z: &str) -> Option<u8> {
    // Sprawdzamy, czy napis jest pusty
    if z.is_empty() {
        return None; // Jeśli napis jest pusty, zwracamy None, ponieważ nie możemy obliczyć wartości z pustego napisu
    }

    // Sprawdzamy, czy w napisie znajdują się tylko cyfry ósemkowe
    // Funkcja `is_digit(8)` sprawdza, czy każda cyfra należy do systemu ósemkowego (0-7)
    if !z.chars().all(|c| c.is_digit(8)) {
        return None; // Jeśli którakolwiek cyfra nie jest cyfrą ósemkową (czyli nie jest w zakresie 0-7), zwracamy None
    }

    // Zamiana z ósemkowego na dziesiętny
    let mut wynik = 0u8; // Zmienna do przechowywania wyniku w systemie dziesiętnym (u8, czyli liczba w zakresie 0-255)
    let mut potega: u32 = 1; // Potęga 8, zaczynamy od 8^0 (czyli 1), która odpowiada za miejsce w systemie ósemkowym

    // Iterujemy po napisie od końca (od najmniej znaczącej cyfry)
    for c in z.chars().rev() { // `rev()` odwraca napis, aby iterować od najmniej znaczącej cyfry
        // `to_digit(10)` konwertuje znak na liczbę dziesiętną (czyli na cyfrę), którą możemy używać do obliczeń
        let cyfra = c.to_digit(10).unwrap_or(0) as u32; // Jeżeli `to_digit` zwróci `None`, traktujemy to jako 0 (choć dla poprawnego napisu nie powinno się to zdarzyć)
        wynik += (cyfra * potega) as u8; // Zwiększamy wynik o wartość tej cyfry pomnożoną przez odpowiednią potęgę ósemki (np. 2 * 8^2 dla 2 na trzeciej pozycji)
        potega *= 8; // Zwiększamy potęgę ósemki (odpowiedzialną za następne miejsce w systemie ósemkowym)
    }

    // Sprawdzamy, czy wynik mieści się w zakresie 0-255 (czyli w zakresie typu `u8`)
    if wynik <= u8::MAX {
        Some(wynik) // Zwracamy wynik, jeśli mieści się w zakresie 0-255
    } else {
        None // Zwracamy None, jeśli wynik jest poza zakresem (więcej niż 255)
    }
}

fn wartosc_syst8(z: &str) -> Option<u8> {
    // Sprawdzamy, czy napis jest pusty
    if z.is_empty() {
        return None; // Jeśli napis jest pusty, zwracamy None
    }

    // Sprawdzamy, czy w napisie znajdują się tylko cyfry ósemkowe
    if !z.chars().all(|c| c.is_digit(8)) {
        return None; // Jeśli w napisie znajdują się znaki spoza zakresu cyfr ósemkowych (0-7), zwracamy None
    }

    // Zamiana z ósemkowego na dziesiętny
    let mut wynik = 0u8; // Wynik w systemie dziesiętnym
    let mut potega: u32 = 1; // Potęga liczby 8

    // Iterujemy po napisie od końca (od najmniej znaczącej cyfry)
    for c in z.chars().rev() { // Odwracamy napis, by operować od najmniej znaczącej cyfry (czyli od prawej strony)
        // `to_digit(10)` konwertuje znak na cyfrę dziesiętną (czyli 0-7, bo w systemie ósemkowym są tylko te cyfry)
        let cyfra = c.to_digit(10)? as u32; // Używamy `?` do propagacji błędów, gdy `to_digit` nie może skonwertować znaku na cyfrę
        wynik += (cyfra * potega) as u8; // Zwiększamy wynik o wartość cyfry pomnożoną przez odpowiednią potęgę 8
        potega *= 8; // Zwiększamy potęgę ósemki
    }

    // Sprawdzamy, czy wynik mieści się w zakresie od 0 do 255 (czyli czy wynik jest poprawny dla typu `u8`)
    if wynik <= u8::MAX {
        Some(wynik) // Zwracamy wynik jako `Some(wynik)` jeśli mieści się w zakresie
    } else {
        None // Zwracamy `None`, jeśli wynik przekroczy zakres
    }
}

fn main() {
    println!("{:?}", wartosc_syst8("10"));  // Some(8)
    println!("{:?}", wartosc_syst8("17"));  // Some(15)
    println!("{:?}", wartosc_syst8("8"));   // None (niepoprawna cyfra)
    println!("{:?}", wartosc_syst8("255")); // Some(173)
    println!("{:?}", wartosc_syst8(""));    // None (pusty napis)
}