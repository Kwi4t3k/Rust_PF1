fn najdluzszy_prefiks(lista: &[String]) -> String {
    // Jeśli lista jest pusta, to nie da się znaleźć wspólnego prefiksu,
    // więc zwracamy pusty napis
    if lista.is_empty() {
        return String::new();
    }

    // Bierzemy pierwszy napis z listy jako punkt odniesienia do porównań
    let pierwszy = &lista[0];

    // Iterujemy po indeksach liter w pierwszym napisie (0, 1, 2, ...)
    for i in 0..pierwszy.len() {
        // Pobieramy literę z pierwszego napisu na pozycji `i`
        let litera = pierwszy.chars().nth(i).unwrap();

        // Przechodzimy przez wszystkie pozostałe napisy w liście, pomijając pierwszy
        for napis in lista.iter().skip(1) {
            // Sprawdzamy dwa warunki:
            // - czy dany napis jest krótszy niż `i` (czyli nie ma już litery na tej pozycji)
            // - czy litera na tej pozycji jest różna od litery z pierwszego napisu
            if napis.len() <= i || napis.chars().nth(i).unwrap() != litera {
                // Jeśli którykolwiek napis nie ma tej samej litery na tej pozycji,
                // kończymy szukanie i zwracamy dotychczasowy prefiks (od 0 do i)
                return pierwszy.chars().take(i).collect();
            }
        }
    }

    // Jeśli wszystkie napisy miały te same litery co pierwszy do samego końca,
    // to zwracamy cały pierwszy napis jako wspólny prefiks
    pierwszy.clone()
}

fn main() {
    let dane1 = vec![
        "programowanie".to_string(),
        "programista".to_string(),
        "program".to_string(),
    ];

    let dane2 = vec![
        "dom".to_string(),
        "drzewo".to_string(),
        "droga".to_string(),
    ];

    let dane3 = vec![
        "abc".to_string(),
        "abc".to_string(),
        "abc".to_string(),
    ];

    let dane4 = vec![
        "niebo".to_string(),
        "ziemia".to_string(),
        "woda".to_string(),
    ];

    println!("Prefiks (1): {}", najdluzszy_prefiks(&dane1)); // "program"
    println!("Prefiks (2): {}", najdluzszy_prefiks(&dane2)); // "d"
    println!("Prefiks (3): {}", najdluzszy_prefiks(&dane3)); // "abc"
    println!("Prefiks (4): {}", najdluzszy_prefiks(&dane4)); // ""
}