Zadania wykonaj w dwóch wersjach — przy użyciu pętli oraz bez ich użycia (z iteratorami zamiast tego).

1. Utwórz (i wyświetl) wektor zawierający:
    - małe litery alfabetu angielskiego;
    - kwadraty 10. kolejnych liczb całkowitych począwszy od 1;
    - 10 kolejnych potęg dwójki;
    - odwrotności wszystkich liczb od 1 do 20;
    - liczby od 1 do 100 podzielne przez 3, ale niepodzielne przez 4.


fn main() {
    let mut vec = Vec::new();

    // 1. Małe litery alfabetu angielskiego
    for i in 'a'..='z' {
        vec.push(i);
    }

    // 2. Kwadraty 10 kolejnych liczb całkowitych począwszy od 1
    for i in 1..=10 {
        vec.push(i * i);
    }

    // 3. 10 kolejnych potęg dwójki
    for i in 0..10 {
        vec.push(2_i32.pow(i));
    }

    // 4. Odwrotności wszystkich liczb od 1 do 20
    for i in 1..=20 {
        vec.push(format!("1/{}", i)); // Zmieniono na formatowanie jako tekst
    }

    // 5. Liczby od 1 do 100 podzielne przez 3, ale niepodzielne przez 4
    for i in 1..=100 {
        if i % 3 == 0 && i % 4 != 0 {
            vec.push(i);
        }
    }

    // Wersja z iteratorami:
    let vec_iter: Vec<_> = ('a'..='z').collect(); // Małe litery alfabetu
    let vec_squares: Vec<_> = (1..=10).map(|i| i * i).collect(); // Kwadraty liczb
    let vec_powers_of_two: Vec<_> = (0..10).map(|i| 2_i32.pow(i)).collect(); // Potęgi dwójki
    let vec_inverses: Vec<_> = (1..=20).map(|i| format!("1/{}", i)).collect(); // Odwrotności
    let vec_div_by_3_not_4: Vec<_> = (1..=100).filter(|&i| i % 3 == 0 && i % 4 != 0).collect(); // Liczby podzielne przez 3, ale nie przez 4

    // Wyświetlenie wyników
    for x in vec_iter {
        print!("{x} ");
    }

    println!();

    for x in vec_squares {
        print!("{x} ");
    }

    println!();

    for x in vec_powers_of_two {
        print!("{x} ");
    }

    println!();

    for x in vec_inverses {
        print!("{x} ");
    }

    println!();

    for x in vec_div_by_3_not_4 {
        print!("{x} ");
    }
}


2. Napisz zestaw funkcji, które dla danego wektora napisów zwrócą:
    - wektor zawierającą napisy krótsze niż 4 znaki;

    fn iterator(vec: Vec<String>) -> Vec<String> {
        vec.into_iter() // Przechodzimy przez wektor, pobierając własność
            .filter(|s| s.len() < 4)  // Filtrujemy napisy krótsze niż 4 znaki
            .collect()  // Zbieramy wynik do nowego wektora
    }
    fn petla(vec: Vec<String>) -> Vec<String> {
        let mut vec1 = Vec::new();

        for i in vec {
            if i.len() < 4 {
                vec1.push(i);
            }
        }

        vec1
    }

    - wektor napisów nie zawierających liter 'a' ani 'A';

fn iterator(vec: Vec<String>) -> Vec<String> {
    vec.into_iter().filter(|c| !c.contains('a') && (c != "A")).collect()
}

fn petla(vec: Vec<String>) -> Vec<String> {
	let mut result = Vec::new();

	for i in vec {
		if !i.contains('a') && !i.contains('A') {
			result.push(i);
		}
	}

	result
}

    - wektor napisów zawierających cyfry;

fn iterator(vec: Vec<String>) -> Vec<String> {
	vec.into_iter().filter(|s| s.chars().any(|c| c.is_digit(10))).collect()
}

fn petla(vec: Vec<String>) -> Vec<String> {
	let mut result = Vec::new();

	for s in vec {
		if s.chars().any(|c| c.is_digit(10)) {
			result.push(s);
		}
	}

	result
}

    - wektor zawierający te same napisy ale odwrócone;

fn iterator(vec: Vec<String>) -> Vec<String> {
    vec.into_iter()  // Przechodzimy przez wektor, biorąc jego własność
        .map(|s| s.chars().rev().collect::<String>())  // Odwracamy każdy napis
        .collect()  // Zbieramy wynik do nowego wektora
}

fn petla(vec: Vec<String>) -> Vec<String> {
    let mut result = Vec::new();

    for s in vec {
        let mut reversed = String::new(); // Pusty napis do przechowywania odwróconego napisu

        // Iterujemy po znakach w napisie od końca do początku
        for c in s.chars().rev() {
            reversed.push(c); // Dodajemy znak do odwróconego napisu
        }

        result.push(reversed); // Dodajemy odwrócony napis do wyniku
    }

    result
}

    - wektor napisów zawierających podwojoną literę (np.: inny, pizza, brutto, lekki, dzienny, itp).

fn iterator(vec: Vec<String>) -> Vec<String> {
    // Tworzymy nowy wektor, filtrując napisy, które zawierają podwójną literę
    vec.into_iter()  // Używamy 'into_iter()' aby iterować po wektorze i przekształcić go w iterator
        .filter(|s| {  // Filtrujemy napisy, sprawdzając każdy z nich
            // Dla każdego napisu wykonujemy zip, aby łączyć każdy znak z jego następnym
            s.chars()
                .zip(s.chars().skip(1))  // 'zip' łączy znak z kolejnym znakiem w napisie
                .any(|(a, b)| a == b)   // 'any' sprawdza, czy którakolwiek para znaków jest taka sama
        })
        .collect()  // Zbieramy wszystkie pasujące napisy do nowego wektora
}

fn petla(vec: Vec<String>) -> Vec<String> {
    let mut result = Vec::new();

    // Przechodzimy przez każdy napis w wektorze
    for s in vec {
        let mut found_double = false; // Zmienna do sprawdzenia, czy znaleźliśmy podwójną literę

        // Iterujemy po znakach w napisie, zaczynając od drugiego znaku
        for i in 1..s.len() {
            if s.chars().nth(i) == s.chars().nth(i - 1) {
                found_double = true; // Jeśli dwie kolejne litery są takie same, ustawiamy flagę
                break; // Przerywamy pętlę, bo już znaleźliśmy podwójną literę
            }
        }

        // Jeśli znaleźliśmy podwójną literę, dodajemy napis do wyników
        if found_double {
            result.push(s);
        }
    }

    result
}

fn main() {
    let test_strings = vec![
		String::from("kot"),
		String::from("pies"),
		String::from("dom"),
		String::from("Ala"),
		String::from("zamek"),
		String::from("król"),
		String::from("robot"),
		String::from("1234"),
		String::from("test1"),
		String::from("inny"),
		String::from("pizza"),
		String::from("brutto"),
		String::from("lekki"),
		String::from("dzienny"),
		String::from("programowanie"),
		String::from("Rust"),
		String::from("wow"),
		String::from("gamma"),
		String::from("delta"),
		String::from("epsilon"),
		String::from("hello123"),
		String::from("abcd"),
		String::from("xyz"),
		String::from("foo"),
		String::from("bar"),
	];

    println!("{:?}", iterator(test_strings.clone()));
    println!("{:?}", petla(test_strings));
}


3. Napisz funkcję
fn indeksy(tablica: ..., element: &str) -> ...
która zwróci wektor indeksów (licząc od zera), na których znajduje się w zadanej tablicy podany element.


fn indeksy_iterator(tablica: Vec<String>, element: &str) -> Vec<usize> {
    tablica.into_iter()
        .enumerate()  // Otrzymujemy (index, value)
        .filter(|(_, x)| x == element)  // Porównujemy wartość z elementem
        .map(|(i, _)| i)  // Bierzemy tylko indeksy
        .collect()  // Zbieramy wyniki do wektora
}

fn indeksy_petla(tablica: Vec<String>, element: &str) -> Vec<usize> {
	let mut result = Vec::new();

	for (i, s) in tablica.iter().enumerate() { // `enumerate()` daje dostęp do indeksu i elementu
		if s == element { // tylko jeśli element jest równy szukanemu napisowi
			result.push(i);
		}
	}

	result
}

fn main() {
    let test_strings = vec![
		String::from("kot"),
		String::from("pies"),
		String::from("dom"),
		String::from("Ala"),
		String::from("zamek"),
		String::from("król"),
		String::from("robot"),
		String::from("1234"),
		String::from("test1"),
		String::from("inny"),
		String::from("pizza"),
		String::from("brutto"),
		String::from("lekki"),
		String::from("dzienny"),
		String::from("programowanie"),
		String::from("Rust"),
		String::from("wow"),
		String::from("gamma"),
		String::from("delta"),
		String::from("epsilon"),
		String::from("hello123"),
		String::from("abcd"),
		String::from("xyz"),
		String::from("foo"),
		String::from("bar"),
		String::from("brutto"),
	];

    println!("{:?}", indeksy_iterator(test_strings.clone(), "brutto"));
    println!("{:?}", indeksy_petla(test_strings, "brutto"));
}