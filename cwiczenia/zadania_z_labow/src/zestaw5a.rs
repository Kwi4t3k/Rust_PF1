// 1

/// Zamienia pojedynczy znak ósemkowy na odpowiadający mu ciąg trzech bitów.
/// Nie wolno tu nic modyfikować!
fn char_2_string(z: char) -> Option<String> {
    // Dopasowujemy znak z do literalnego łańcucha bitów
    let u = match z {
        '0' => Some("000"),   // '0' → "000"
        '1' => Some("001"),   // '1' → "001"
        '2' => Some("010"),   // '2' → "010"
        '3' => Some("011"),   // '3' → "011"
        '4' => Some("100"),   // '4' → "100"
        '5' => Some("101"),   // '5' → "101"
        '6' => Some("110"),   // '6' → "110"
        '7' => Some("111"),   // '7' → "111"
        _   => None,          // każdy inny znak → błąd (None)
    };

    // Przekształcamy Option<&str> w Option<String>
    match u {
        Some(s) => Some(String::from(s)), // Some("&str") → Some(String)
        None    => None,                  // None → None
    }
}

/// Wersja **bez** operatora `?`
fn zamien_sys8_na_sys2_bez_pyt(z: &str) -> Option<String> {
    // Jeśli podano pusty łańcuch, zwracamy None
    if z.is_empty() {
        return None;
    }
    // Przygotowujemy pusty String na kolejne bity
    let mut bity = String::new();
    // Iterujemy po każdym znaku w łańcuchu
    for c in z.chars() {
        match char_2_string(c) {
            Some(trzy_bity) => bity.push_str(&trzy_bity), // doklejamy otrzymane 3 bity
            None            => return None,               // niepoprawny znak → None
        }
    }
    // Usuwamy wiodące zera z połączonych bitów
    let obciete = bity.trim_start_matches('0');
    if obciete.is_empty() {
        // Jeśli po obcięciu nic nie zostało, wynik samymi zerami → "0"
        Some("0".to_string())
    } else {
        // W przeciwnym razie zwracamy obcięty ciąg
        Some(obciete.to_string())
    }
}

/// Wersja **z** operatorem `?`
fn zamien_sys8_na_sys2(z: &str) -> Option<String> {
    // Pusty parametr → None
    if z.is_empty() {
        return None;
    }
    // Przygotowujemy pusty String na kolejne bity
    let mut bity = String::new();
    // Iterujemy po znakach wejściowego łańcucha
    for c in z.chars() {
        // Wywołujemy helper; jeśli zwróci None, cała funkcja kończy się None
        let trzy = char_2_string(c)?;
        bity.push_str(&trzy); // doklejamy 3-bity
    }
    // Usuwamy wiodące zera
    let obciete = bity.trim_start_matches('0');
    if obciete.is_empty() {
        // Jeśli wszystko było zerami, zwracamy pojedyncze "0"
        Some("0".to_string())
    } else {
        // W innym wypadku obcięty ciąg
        Some(obciete.to_string())
    }
}

fn main() {
    // Kilka podstawowych testów – drukujemy wejście i wynik wywołania:

    // pusty → None
    println!("zamien_sys8_na_sys2(\"\") = {:?}", zamien_sys8_na_sys2(""));
    assert_eq!(zamien_sys8_na_sys2(""), None);

    // nie-ósemkowy → None
    println!("zamien_sys8_na_sys2(\"8\") = {:?}", zamien_sys8_na_sys2("8"));
    assert_eq!(zamien_sys8_na_sys2("8"), None);

    // "0" → "0"
    println!("zamien_sys8_na_sys2(\"0\") = {:?}", zamien_sys8_na_sys2("0"));
    assert_eq!(zamien_sys8_na_sys2("0"), Some("0".into()));

    // "1" → "1"
    println!("zamien_sys8_na_sys2(\"1\") = {:?}", zamien_sys8_na_sys2("1"));
    assert_eq!(zamien_sys8_na_sys2("1"), Some("1".into()));

    // "2" → "10"
    println!("zamien_sys8_na_sys2(\"2\") = {:?}", zamien_sys8_na_sys2("2"));
    assert_eq!(zamien_sys8_na_sys2("2"), Some("10".into()));

    // "7" → "111"
    println!("zamien_sys8_na_sys2(\"7\") = {:?}", zamien_sys8_na_sys2("7"));
    assert_eq!(zamien_sys8_na_sys2("7"), Some("111".into()));

    // "10" → "1000"
    println!("zamien_sys8_na_sys2(\"10\") = {:?}", zamien_sys8_na_sys2("10"));
    assert_eq!(zamien_sys8_na_sys2("10"), Some("1000".into()));

    // "17" → "1111"
    println!("zamien_sys8_na_sys2(\"17\") = {:?}", zamien_sys8_na_sys2("17"));
    assert_eq!(zamien_sys8_na_sys2("17"), Some("1111".into()));

    // "20" → "10000"
    println!("zamien_sys8_na_sys2(\"20\") = {:?}", zamien_sys8_na_sys2("20"));
    assert_eq!(zamien_sys8_na_sys2("20"), Some("10000".into()));

    // sprawdzamy też wersję bez operatora `?`
    println!(
        "zamien_sys8_na_sys2_bez_pyt(\"17\") = {:?}",
        zamien_sys8_na_sys2_bez_pyt("17")
    );
    assert_eq!(zamien_sys8_na_sys2_bez_pyt("17"), Some("1111".into()));
}

// 2

/// Pomocnicza funkcja – zamienia znak '0' lub '1' na odpowiadający mu bit (0 lub 1).
/// Dla każdego innego znaku zwraca None.
fn znak_na_bit(znak: char) -> Option<u8> {
    match znak {
        '0' => Some(0),  // '0' → bit 0
        '1' => Some(1),  // '1' → bit 1
        _   => None,     // wszystko inne → nie jest cyfrą dwójkową
    }
}

/// Wersja **bez** operatora `?`
fn wartosc_sys2_bez_pytajnika(napis: &str) -> Option<u8> {
    if napis.is_empty() {
        return None;                  // pusty łańcuch → błąd (None)
    }
    let mut suma: u16 = 0;            // używamy u16, by wykryć przepełnienie ponad 8 bitów
    for znak in napis.chars() {       // iterujemy po znakach łańcucha
        // tłumaczymy pojedynczy znak na bit albo zwracamy None
        let bit = match znak_na_bit(znak) {
            Some(b) => b,             // poprawny bit
            None    => return None,   // nie-cyfra dwójkowa → None
        };
        suma = suma * 2 + (bit as u16);   // dodajemy nowy bit (shift + OR)
        if suma > (u8::MAX as u16) {      // jeśli wynik wykracza poza 8 bitów (255)
            return None;                  // → błąd (None)
        }
    }
    Some(suma as u8)  // konwertujemy na u8 i zwracamy w Some
}

/// Wersja **z** operatorem `?`
fn wartosc_sys2(napis: &str) -> Option<u8> {
    if napis.is_empty() {
        return None;                  // pusty łańcuch → None
    }
    let mut suma: u16 = 0;           // akumulator w u16 dla bezpieczeństwa
    for znak in napis.chars() {
        let bit = znak_na_bit(znak)?;     // próba pobrania bitu, ? od razu zwróci None przy błędzie
        suma = suma * 2 + (bit as u16);   // shift w lewo i dodanie bitu
        if suma > (u8::MAX as u16) {      // sprawdzamy, czy nie przekroczyliśmy 255
            return None;                  // jeśli tak, zwracamy None
        }
    }
    Some(suma as u8)  // jeśli wszystko OK, zwracamy wynik jako u8
}

fn main() {
    // Przykładowe wywołania:
    println!("{:?}", wartosc_sys2(""));         // → None (pusty)
    println!("{:?}", wartosc_sys2("2"));        // → None (nie-binarna cyfra)
    println!("{:?}", wartosc_sys2("101"));      // → Some(5)
    println!("{:?}", wartosc_sys2("11111111")); // → Some(255)
    println!("{:?}", wartosc_sys2("100000000")); // → None (256 ≫ 8 bitów)
}

// 3

/// Pomocnicza funkcja – zamienia znak ósemkowy ('0'..='7') na odpowiadającą mu wartość 0–7.
/// Dla każdego innego znaku zwraca None.
fn znak_na_osemke(znak: char) -> Option<u8> {
    match znak {
        '0' => Some(0),
        '1' => Some(1),
        '2' => Some(2),
        '3' => Some(3),
        '4' => Some(4),
        '5' => Some(5),
        '6' => Some(6),
        '7' => Some(7),
        _   => None,       // nie-ósemkowy znak → błąd
    }
}

/// Wersja **bez** operatora `?`
fn wartosc_syst8_bez_pyt(z: &str) -> Option<u8> {
    // 1) pusty łańcuch = None
    if z.is_empty() {
        return None;
    }
    // 2) akumulator w u16, by wykryć przepełnienie > 255
    let mut suma: u16 = 0;
    // 3) iterujemy po każdej cyfrze ósemkowej
    for znak in z.chars() {
        // mapujemy '0'..'7' → 0..7 lub zwracamy None
        let cyfra = match znak_na_osemke(znak) {
            Some(d) => d,
            None    => return None,     // niepoprawny znak
        };
        // dodajemy kolejną cyfrę: shift *8 plus nowa wartość
        suma = suma * 8 + (cyfra as u16);
        // jeżeli przekroczyliśmy zakres 0..=255 → None
        if suma > (u8::MAX as u16) {
            return None;
        }
    }
    // 4) wynik mieści się w u8 → zwracamy Some(u8)
    Some(suma as u8)
}

/// Wersja **z** operatorem `?`
fn wartosc_syst8(z: &str) -> Option<u8> {
    // 1) pusty łańcuch = None
    if z.is_empty() {
        return None;
    }
    // 2) akumulator w u16
    let mut suma: u16 = 0;
    // 3) iterujemy po znakach
    for znak in z.chars() {
        // konwertujemy lub od razu zwracamy None
        let cyfra = znak_na_osemke(znak)?;
        // shift ośmiokrotny + wartość
        suma = suma * 8 + (cyfra as u16);
        // jeżeli >255, to None
        if suma > (u8::MAX as u16) {
            return None;
        }
    }
    // 4) zwracamy wynik jako u8
    Some(suma as u8)
}

fn main() {
    // testy
    println!("wartosc_syst8(\"\") = {:?}", wartosc_syst8(""));          // None
    println!("wartosc_syst8(\"8\") = {:?}", wartosc_syst8("8"));        // None (zły znak)
    println!("wartosc_syst8(\"10\") = {:?}", wartosc_syst8("10"));      // Some(8)
    println!("wartosc_syst8(\"377\") = {:?}", wartosc_syst8("377"));    // Some(255)
    println!("wartosc_syst8(\"400\") = {:?}", wartosc_syst8("400"));    // None (256 > 255)
    // oraz wersja bez pytajnika:
    println!(
        "bez_pyt wartosc_syst8_bez_pyt(\"17\") = {:?}",
        wartosc_syst8_bez_pyt("17")
    ); // Some(15)
}