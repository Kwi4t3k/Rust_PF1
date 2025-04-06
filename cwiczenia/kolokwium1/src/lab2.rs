fn silnia_while(mut liczba: i32) -> i32 {
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

fn silnia_for(mut liczba: i32) -> i32 {
    if liczba == 0 {
        1
    } else {
        let mut wynik = 1;

        for i in 1..=liczba {
            wynik *= i;
        }

        wynik
    }
}

fn silnia_loop(mut liczba: i32) -> i32 {
    if liczba == 0 {
        1
    } else {
        let mut wynik = 1;
        loop {
            if liczba < 1 {
                break;
            }

            wynik *= liczba;
            liczba -= 1;
        }
        wynik
    }
}

fn trojki_pitagorejskie_do_danej_liczby_while(liczba: i32) {
    let mut c = 1;

    while c <= liczba {
        let mut b = 1;

        while b < c {
            let mut a = 1;

            while a < b {
                if a * a + b * b == c * c {
                    println!("{a} {b} {c}");
                }
                a += 1;
            }
            b += 1;
        }
        c += 1;
    }
}

fn trojki_pitagorejskie_do_danej_liczby_for(liczba: i32) {
    for c in 1..=liczba {
        for b in 1..c {
            for a in 1..b {
                if a * a + b * b == c * c {
                    println!("{a} {b} {c}");
                }
            }
        }
    }
}

fn trojki_pitagorejskie_do_danej_liczby_loop(liczba: i32) {
    let mut c = 1;

    loop {
        let mut b = 1;

        if c > liczba {
            break;
        }

        loop {
            let mut a = 1;

            if b >= c {
               break;
            }

            loop {
                if a >= b {
                    break;
                }

                if a * a + b * b == c * c {
                    println!("{a} {b} {c}");
                }
                a += 1;
            }
            b += 1;
        }
        c += 1;
    }
}

fn trojki_pitagorejskie_rek(a: i32, b: i32, c: i32, max: i32) {
    if c > max {
        return;
    }

    if b >= c {
        trojki_pitagorejskie_rek(1, 1, c + 1, max);
        return;
    }

    if a >= b {
        trojki_pitagorejskie_rek(1, b + 1, c, max);
        return;
    }

    if a * a + b * b == c * c {
        println!("{a} {b} {c}");
    }

    trojki_pitagorejskie_rek(a + 1, b, c, max);
}

fn main() {
    println!("{}", silnia_while(0)); // 1
    println!("{}", silnia_while(5)); // 120

    println!("-----------------");

    println!("{}", silnia_for(0)); // 1
    println!("{}", silnia_for(5)); // 120

    println!("-----------------");

    println!("{}", silnia_loop(0)); // 1
    println!("{}", silnia_loop(5)); // 120

    println!("-----------------");

    trojki_pitagorejskie_do_danej_liczby_while(10);

    println!("-----------------");

    trojki_pitagorejskie_do_danej_liczby_for(10);

    println!("-----------------");

    trojki_pitagorejskie_do_danej_liczby_loop(10);

    println!("-----------------");
}

fn tabela_ascii() {
    for i in 33..=126 {
        println!("{} {}", i, i as u8 as char);
    }
}

fn problem_collatza(mut n: i32) -> i32 {
    let mut numer_iteracji = 0;

    loop {
        if n == 1 {
            break;
        }

        if n % 2 == 0 {
            n /= 2;
            numer_iteracji += 1;
        } else if n % 2 != 0 {
            n = 3 * n + 1;
            numer_iteracji += 1;
        }
    }

    numer_iteracji
}

fn czy_liczba_armstronga(liczba: i32) -> bool {
    let mut liczba_copy = liczba;
    let mut liczby = Vec::new();

    while liczba_copy != 0 {
        liczby.push(liczba_copy % 10);
        liczba_copy /= 10;
    }

    let mut wynik_copy = 0;
    for i in &liczby {
        wynik_copy += i.pow(liczby.len() as u32);
    }

    wynik_copy == liczba
}

fn czy_liczba_doskonala(liczba: i32) -> bool {
    let mut suma = 0;

    for i in 1..liczba {
        if liczba % i == 0 {
            suma += i;
        }
    }

    suma == liczba
}

fn rozklad_na_czynniki_pierwsze(mut liczba: i32) -> String {
    let mut tab = String::new();
    let mut i = 2;

    while liczba != 1 {

        if liczba % i == 0 {
            tab.push_str(&i.to_string());
            tab.push_str("x");

            liczba /= i;
        } else {
            i += 1;
        }
    }

    if tab.ends_with('x') {
        tab.pop();
    }

    tab
}

fn pow_mod(x: u128, n: u128, p: u128) -> u128 {
    if n == 0 {
        1
    } else if n % 2 == 0 {
        let half = pow_mod(x, n / 2, p);
        (half * half) % p
    } else {
        (x % p * pow_mod(x, n - 1, p)) % p
    }
}

fn main() {
    tabela_ascii();
    println!();

    println!("{}", problem_collatza(12)); // 9

    println!("{}", czy_liczba_armstronga(153)); // true
    println!("{}", czy_liczba_armstronga(1634)); // true

    println!("--------------");

    println!("{}", czy_liczba_doskonala(6));   // true
    println!("{}", czy_liczba_doskonala(28));  // true
    println!("{}", czy_liczba_doskonala(12));  // false

    println!("--------------");

    println!("{}", rozklad_na_czynniki_pierwsze(72));  // 2x2x2x3x3

    println!("--------------");

    println!("{}", pow_mod(2, 10, 1000)); // 24
    println!("{}", pow_mod(123456789, 123456, 1000000007));
}