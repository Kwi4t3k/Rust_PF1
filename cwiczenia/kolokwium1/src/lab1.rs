fn czy_rok_przestepny(rok: i32) -> bool {
    if (rok % 4 == 0 && rok % 100 != 0) || rok % 400 == 0 {
        return true;
    }
    false
}

fn liczba_dni_miesiaca(miesiac: i32, rok: i32) -> i32 {
    match miesiac {
        1 | 3 | 5 | 7 | 8 | 10 | 12 => 31,
        4 | 6 | 9 | 11 => 30,
        2 if czy_rok_przestepny(rok) => 29,
        2 => 28,
        _ => 0,
    }
}

fn celsjusz_na_fahrenheit(temperatura: i32) -> f32 {
    32.0 + (9.0 / 5.0) * temperatura as f32
}

fn fahrenheit_na_celsjusz(temperatura: f32) -> i32 {
    (((temperatura - 32.0) * 5.0) / 9.0).round() as i32
}

fn roznica_czasow(h1: i32, m1: i32, s1: i32, h2: i32, m2: i32, s2: i32) -> String {
    let czas1 = h1 * 3600 + m1 * 60 + s1;
    let czas2 = h2 * 3600 + m2 * 60 + s2;

    let mut wynik = czas2 - czas1;

    if wynik < 0 {
        wynik = -wynik;
    }

    let h_wynik = wynik / 3600;
    let m_wynik = (wynik % 3600) / 60;
    let s_wynik = wynik % 60;

    let mut result = String::new();

    result.push_str(&h_wynik.to_string());
    result.push_str(" : ");
    result.push_str(&m_wynik.to_string());
    result.push_str(" : ");
    result.push_str(&s_wynik.to_string());

    result
}

fn silnia(mut liczba: i32) -> i32 {
    if liczba == 0 {
        1
    } else {
        let mut wynik = 1;
        while liczba > 0 {
            wynik *= liczba;
            liczba -= 1;
        }
        wynik
    }
}

fn cyfry_liczby_od_konca(mut liczba: i32) {
    while liczba != 0 {
        print!("{}", liczba % 10);
        liczba /= 10;
    }
}

fn suma_cyfr_danej_liczby(mut liczba: i32) -> i32 {
    let mut wynik = 0;

    while liczba != 0 {
        wynik += liczba % 10;
        liczba /= 10;
    }

    wynik
}

fn trojki_pitagorejskie_do_danej_liczby(liczba: i32) {
    for c in 1..=liczba {
        for b in 1..c {
            for a in 1..b {
                if a*a + b*b == c*c {
                    println!("{a} {b} {c}");
                }
            }
        }
    }
}

fn main() {
    print!("{}", czy_rok_przestepny(2020)); // true
    println!("    {}", czy_rok_przestepny(1900)); // false

    print!("{}", liczba_dni_miesiaca(2, 2020));         // 29
    print!("    {}", liczba_dni_miesiaca(2, 2019));     // 28
    print!("    {}", liczba_dni_miesiaca(9, 2019));     // 30
    println!("    {}", liczba_dni_miesiaca(3, 2019));   // 31

    print!("{}", celsjusz_na_fahrenheit(0)); // 32
    println!("  {}", celsjusz_na_fahrenheit(1)); // 33.8

    print!("{}", fahrenheit_na_celsjusz(32.0)); // 0
    println!("  {}", fahrenheit_na_celsjusz(33.8)); // 1

    println!("{}", roznica_czasow(12, 30, 15, 14, 0, 0)); // 01 : 29 : 45

    print!("{}", silnia(0)); // 1
    println!("  {}", silnia(5)); // 120

    cyfry_liczby_od_konca(123456);

    println!("\n{}", suma_cyfr_danej_liczby(12345));

    println!();
    trojki_pitagorejskie_do_danej_liczby(10);
}