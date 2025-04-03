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