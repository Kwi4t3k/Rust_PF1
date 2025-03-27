fn wartosc_syst8(z: &str) -> Option<u8> {
    // Sprawdzamy, czy napis jest pusty
    if z.is_empty() {
        return None;
    }

    // Sprawdzamy, czy w napisie znajdują się tylko cyfry ósemkowe
    if !z.chars().all(|c| c.is_digit(8)) {
        return None; // Jeśli którakolwiek cyfra nie należy do systemu ósemkowego, zwracamy None
    }

    // Zamiana z ósemkowego na dziesiętny
    let mut wynik = 0u8;
    let mut potega: u32 = 1;

    // Iterujemy po napisie od końca (od najmniej znaczącej cyfry)
    for c in z.chars().rev() {
        let cyfra = c.to_digit(10).unwrap_or(0) as u32;
        wynik += (cyfra * potega) as u8;
        potega *= 8;
    }

    // Sprawdzamy, czy wynik mieści się w zakresie 0-255
    if wynik <= u8::MAX {
        Some(wynik) // Zwracamy wynik, jeśli mieści się w zakresie 0-255
    } else {
        None // Zwracamy None, jeśli wynik jest poza zakresem
    }
}

fn wartosc_syst8(z: &str) -> Option<u8> {
    // Sprawdzamy, czy napis jest pusty
    if z.is_empty() {
        return None;
    }

    // Sprawdzamy, czy w napisie znajdują się tylko cyfry ósemkowe
    if !z.chars().all(|c| c.is_digit(8)) {
        return None; // Jeśli którakolwiek cyfra nie należy do systemu ósemkowego, zwracamy None
    }

    // Zamiana z ósemkowego na dziesiętny
    let mut wynik = 0u8;
    let mut potega: u32 = 1;

    // Iterujemy po napisie od końca (od najmniej znaczącej cyfry)
    for c in z.chars().rev() {
        let cyfra = c.to_digit(10)? as u32;  // Używamy ? dla propagacji błędów
        wynik += (cyfra * potega) as u8;
        potega *= 8;
    }

    // Sprawdzamy, czy wynik mieści się w zakresie 0-255
    if wynik <= u8::MAX {
        Some(wynik) // Zwracamy wynik, jeśli mieści się w zakresie 0-255
    } else {
        None // Zwracamy None, jeśli wynik jest poza zakresem
    }
}

fn main() {
    println!("{:?}", wartosc_syst8("10"));  // Some(8)
    println!("{:?}", wartosc_syst8("17"));  // Some(15)
    println!("{:?}", wartosc_syst8("8"));   // None (niepoprawna cyfra)
    println!("{:?}", wartosc_syst8("255")); // Some(173)
    println!("{:?}", wartosc_syst8(""));    // None (pusty napis)
}