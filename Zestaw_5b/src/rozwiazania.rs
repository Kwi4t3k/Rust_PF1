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





3. Napisz funkcję
fn wartosc_cyfry_rzymskiej(c: char) -> Result<u16, String>
która zwraca wartość cyfry rzymskiej podanej jako znak — albo opis tekstowy błędu, jesli znak nie jest cyfrą rzymską (jednym z: I V X L C D M).





4. Napisz funkcję o nagłówku
fn rzymskie(napis: &str) -> Result<u128, String>
która dla napisu reprezentującego liczbę w zapisie rzymskim (nie zakładamy jego poprawności; ponadto pusty ciąg takżejest nie poprawny) zwraca liczbę reprezentowaną przez ów napis lub napisowy opis błędu. Błędy mogą być trojakie — niewłaściwa cyfra (o tym powiadomi nas poniższa funkcja pomocnicza); pusty napis; niewłaściwa kolejność cyfr.
Uwaga! Użyj funkcji z poprzedniego zadania i operatora ?