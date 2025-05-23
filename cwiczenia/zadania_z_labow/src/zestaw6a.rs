// 1

fn main() {
    let mut litery = Vec::new();
    litery = ('a'..='z').collect();

    for c in 'a'..='z' {
        litery.push(c);
    }
    println!("{:?}", litery);

    let mut liczby = Vec::new();
    liczby = (1..=10).map(|i| i*i).collect();

    for i in 1..=10 {
        liczby.push(i*i);
    }
    println!("{:?}", liczby);

    let mut liczbya = Vec::new();
    liczbya = (1..=10).map(|i| 2_i32.pow(i)).collect();

    for i in 1..=10 {
        liczbya.push(2_i32.pow(i));
    }
    println!("{:?}", liczbya);

    let mut liczbyb = Vec::new();
    liczbyb = (1..=20).map(|i| format!("1/{}", i)).collect();

    for i in 1..=20 {
        liczbyb.push(format!("1/{}", i));
    }
    println!("{:?}", liczbyb);

    let mut liczbyc = Vec::new();
    liczbyc = (1..=100).filter(|i| i % 3 == 0 && i % 4 != 0).collect();

    for i in 1..=100 {
        if i % 3 == 0 && i % 4 != 0 {
            liczbyc.push(i);
        }
    }
    println!("{:?}", liczbyc);
}

// 2

fn napisy_krotsze_niz_4(vec: Vec<String>) -> Vec<String> {
    // vec.into_iter().filter(|s| s.len() < 4).collect()

    let mut wynik = Vec::new();
    for s in vec {
        if s.len() < 4 {
            wynik.push(s);
        }
    }
    wynik
}

fn bez_liter_a_A(vec: Vec<String>) -> Vec<String> {
    // vec.into_iter().filter(|s| !s.contains('a') && !s.contains('A')).collect()

    let mut wynik = Vec::new();
    for s in vec {
        if !s.contains('a') && !s.contains('A') {
            wynik.push(s);
        }
    }
    wynik
}

fn napis_zawierajacy_cyfry(vec: Vec<String>) -> Vec<String> {
    // vec.into_iter().filter(|s| s.chars().any(|c| c.is_digit(10))).collect()

    let mut wynik = Vec::new();
    for s in vec {
        if s.chars().any(|c| c.is_digit(10)) {
            wynik.push(s);
        }
    }
    wynik
}

fn odwrocone_napisy(vec: Vec<String>) -> Vec<String> {
    // vec.into_iter().map(|s| s.chars().rev().collect::<String>()).collect()

    let mut wynik = Vec::new();
    for s in vec {
        let mut tmp = "".to_string();
        for c in s.chars().rev() {
            tmp.push(c);
        }
        wynik.push(tmp);
    }
    wynik
}

fn podwojona_litera(vec: Vec<String>) -> Vec<String> {
    // vec.into_iter().filter(|s| {
    //     let mut chars = s.chars().peekable();
    //     while let Some(c) = chars.next() {
    //         if let Some(&next) = chars.peek() {
    //             if c == next {
    //                 return true;
    //             }
    //         }
    //     }
    //     false
    // }).collect()

    let mut wynik = Vec::new();
    for s in vec {
        let bytes = s.as_bytes();
        let mut jest = false;

        for c in 1..bytes.len() {
            if bytes[c] == bytes[c-1] {
                jest = true;
                break;
            }
        }

        if jest {
            wynik.push(s);
        }
    }
    wynik
}

fn main() {
    let napisy = vec![
        String::from("kot"),
        String::from("Ala"),
        String::from("rust"),
        String::from("pizza"),
        String::from("lekki"),
        String::from("dzienny"),
        String::from("123abc"),
        String::from("m4in"),
        String::from("nope"),
        String::from("xy"),
        String::from("aa"),
    ];

    println!("{:?}", napisy_krotsze_niz_4(napisy.clone())); // ["kot", "Ala", "xy", "aa"]
    println!("{:?}", bez_liter_a_A(napisy.clone())); // ["rust", "lekki", "dzienny", "m4in", "nope", "xy"]
    println!("{:?}", napis_zawierajacy_cyfry(napisy.clone())); // ["123abc", "m4in"]
    println!("{:?}", odwrocone_napisy(napisy.clone())); // ["tok", "alA", "tsur", "azzip", "ikkel", "ynneizd", "cba321", "ni4m", "epon", "yx", "aa"]
    println!("{:?}", podwojona_litera(napisy.clone())); // ["pizza", "lekki", "dzienny", "aa"]
}

// 3

fn indeksy(tablica: Vec<String>, element: &str) -> Vec<usize> {
    tablica.into_iter().enumerate().filter(|(_,e)| e == element).map(|(i,_)| i).collect()

    let mut wynik = Vec::new();
    for (i,e) in tablica.iter().enumerate() {
        if e == element {
            wynik.push(i);
        }
    }
    wynik
}

fn main() {
    let tablica= vec![
        String::from("a"),
        String::from("b"),
        String::from("c"),
        String::from("a"),
        String::from("d"),
        String::from("e"),
        String::from("f"),
        String::from("a"),
        String::from("g"),
        String::from("t"),
        String::from("z"),
    ];

    println!("{:?}", indeksy(tablica, "a"));
}