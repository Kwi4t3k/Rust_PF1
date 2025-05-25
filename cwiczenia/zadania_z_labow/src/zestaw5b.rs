// 1

fn wartosc_cyfry(c: char) -> Result<u8, String> {
    match c.to_digit(10) {
        None => Err("To nie jest cyfra dziesiętna".to_string()),
        Some(d) => Ok(d as u8),
    }

    c.to_digit(10).map(|d| d as u8).ok_or_else(|| "To nie jest cyfra dziesiętna".to_string())
}

fn wartosc_cyfry(c: char) -> Result<u8, String> {
    let d = c.to_digit(10).ok_or_else(|| "To nie jest cyfra dziesiętna".to_string())?;

    Ok(d as u8)
}


fn main() {
    let testy = ['0', '5', '9', 'a', '/', '7'];

    for &znak in &testy {
        match wartosc_cyfry(znak) {
            Ok(w) => println!("'{}' → {}", znak, w),
            Err(e) => println!("'{}' → Błąd: {}", znak, e),
        }
    }
    // '0' → 0
    // '5' → 5
    // '9' → 9
    // 'a' → Błąd: To nie jest cyfra dziesiętna
    // '/' → Błąd: To nie jest cyfra dziesiętna
    // '7' → 7
}

// 2

/// Zwraca wartość znaku '0'..'9' jako u8 albo Err, gdy to nie cyfra.
fn wartosc_cyfry(c: char) -> Result<u8, String> {
    c.to_digit(10)
     .map(|d| d as u8)
     .ok_or_else(|| "To nie jest cyfra dziesiętna".to_string())
}

/// Dodaje dwie niepuste liczby zapisane jako napisy dziesiętne „pisemnie”.
/// Zwraca Err, gdy któryś argument jest pusty lub zawiera nie-cyfrę.
fn dodaj_pisemnie(a: &str, b: &str) -> Result<String, String> {
    // 1) Oba napisy muszą być niepuste
    if a.is_empty() || b.is_empty() {
        return Err("Pusty napis".to_string());
    }

    // 2) Zamiana każdego znaku na cyfrę u8
    let mut va = Vec::new();
    for ch in a.chars() {
        va.push(wartosc_cyfry(ch)?);
    }
    let mut vb = Vec::new();
    for ch in b.chars() {
        vb.push(wartosc_cyfry(ch)?);
    }

    // 3) Dodawanie pisemne od końca z przeniesieniem
    let mut wynik = String::new();
    let mut przeniesienie = 0;
    let mut i = va.len() as isize - 1;
    let mut j = vb.len() as isize - 1;
    while i >= 0 || j >= 0 || przeniesienie > 0 {
        // pobieramy cyfra lub 0 jeżeli indeks wyszedł poza
        let da = if i >= 0 { va[i as usize] } else { 0 };
        let db = if j >= 0 { vb[j as usize] } else { 0 };
        let suma = da + db + przeniesienie;
        let cyfra = suma % 10;
        przeniesienie = suma / 10;
        // doklejamy znak najmłodszy; odwrócimy później
        wynik.push(std::char::from_digit(cyfra as u32, 10).unwrap());
        i -= 1;
        j -= 1;
    }

    // 4) Odwrócenie kolejności, żeby dostać wynik prawidłowo
    let wynik: String = wynik.chars().rev().collect();
    Ok(wynik)
}

/// Wersja **z** operatorem `?` dla dodawania pisemnego
fn dodaj_pisemnie(a: &str, b: &str) -> Result<String, String> {
    // 1) sprawdź niepustość
    if a.is_empty() || b.is_empty() {
        return Err("Pusty napis".to_string());
    }

    // 2) sparsuj oba łańcuchy na wektory cyfr, `?` przepuści Err z wartosc_cyfry
    let da: Vec<u8> = a.chars()
        .map(wartosc_cyfry)               // char -> Result<u8, String>
        .collect::<Result<_, _>>()?;      // Result<Vec<u8>, String>

    let db: Vec<u8> = b.chars()
        .map(wartosc_cyfry)
        .collect::<Result<_, _>>()?;

    // 3) dodawanie od końca z przeniesieniem
    let mut res = String::new();
    let mut carry = 0;
    let mut i = da.len() as isize - 1;
    let mut j = db.len() as isize - 1;

    while i >= 0 || j >= 0 || carry > 0 {
        let x = if i >= 0 { da[i as usize] } else { 0 };
        let y = if j >= 0 { db[j as usize] } else { 0 };
        let sum = x + y + carry;
        let digit = sum % 10;
        carry = sum / 10;
        // pushujemy najmłodszą cyfrę (odwrócimy na koniec)
        res.push(std::char::from_digit(digit as u32, 10).unwrap());
        i -= 1;
        j -= 1;
    }

    // 4) odwracamy kolejność, by uzyskać prawidłowy wynik
    Ok(res.chars().rev().collect())
}

fn main() {
    // Przykładowe użycie:
    println!("{:?}", dodaj_pisemnie("123", "456")); // Ok("579")
    println!("{:?}", dodaj_pisemnie("999", "1"));   // Ok("1000")
    println!("{:?}", dodaj_pisemnie("", "12"));     // Err("Pusty napis")
    println!("{:?}", dodaj_pisemnie("12a", "3"));   // Err("To nie jest cyfra dziesiętna")
}