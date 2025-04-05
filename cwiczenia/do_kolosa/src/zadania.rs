### Zadanie 1: Mini system magazynowy
Napisz zestaw funkcji do obsługi prostego magazynu, który będzie reprezentowany jako Vec<String> zawierający nazwy produktów. Zaimplementuj następujące funkcje:

fn dodaj(magazyn: &mut Vec<String>, produkt: String)
– dodaje nowy produkt do magazynu,

fn wypozycz(magazyn: &mut Vec<String>, produkt: &str) -> bool
– usuwa produkt z magazynu (jeśli istnieje) i zwraca true; w przeciwnym razie false,

fn wypisz(magazyn: &Vec<String>)
– wypisuje zawartość magazynu, po jednej pozycji w każdej linii.


fn dodaj(magazyn: &mut Vec<String>, produkt: String) {
    magazyn.push(produkt);
}

fn wypozycz(magazyn: &mut Vec<String>, produkt: &str) -> bool {
    for i in 0..magazyn.len() {
        if magazyn[i] == produkt {
            magazyn.remove(i);
            return true;
        }
    }
    false
}

fn wypisz(magazyn: &Vec<String>) {
    println!("{:?}", magazyn);
}

fn main() {
    let mut magazyn = vec![
        String::from("produkt1"),
        String::from("produkt2"),
        String::from("produkt3"),
        String::from("produkt4"),
        String::from("produkt5"),
        String::from("produkt6"),
    ];

    wypisz(&magazyn);

    wypozycz(&mut magazyn, "produkt2");

    wypisz(&magazyn);

    dodaj(&mut magazyn, String::from("dupa"));

    wypisz(&magazyn);
}


---

### **Zadanie 2: Odwracanie słów**
Napisz funkcję `odwroc_slowa(napis: &str) -> String`, która odwróci kolejność słów w napisie. Przykład:
`"Ala ma kota"` → `"kota ma Ala"`

fn odwroc_slowa(napis: &str) -> String {
    napis.split_whitespace().rev().collect::<Vec<&str>>().join(" ")
}

fn odwroc_slowa(napis: &str) -> String {
    let slowa: Vec<&str> = napis.split_whitespace().collect();
    let mut wynik = String::new();

    let mut i = slowa.len();

    while i > 0 {
        i -= 1;
        wynik.push_str(slowa[i]);

        if i > 0 {
            wynik.push(' ');
        }
    }

    wynik
}

fn main() {
    let napis = "Ala ma kota";
    println!("{}", odwroc_slowa(napis));
}

---

### **Zadanie 3: Filtrowanie liczb**
Napisz funkcję `filtruj(tab: &Vec<i32>) -> Vec<i32>`, która zwróci nowy wektor zawierający tylko liczby **podzielne przez 3, ale niepodzielne przez 4**.

fn filtruj(tab: &Vec<i32>) -> Vec<i32> {
    let mut result: Vec<i32> = Vec::new();

    for i in 0..tab.len() {
        if (tab[i] % 3 == 0) && !(tab[i] % 4 == 0) {
            result.push(tab[i]);
        }
    }

    result
}

fn filtruj(tab: &Vec<i32>) -> Vec<i32> {
    let mut wynik = Vec::new();

    for liczba in tab {
        if liczba % 3 == 0 && liczba % 4 != 0 {
            wynik.push(*liczba);
        }
    }

    wynik
}

fn main() {
    let dane = vec![1, 3, 4, 6, 8, 9, 12, 15, 16, 18, 20, 21, 24, 27, 30];
    let wynik = filtruj(&dane);
    println!("Liczby podzielne przez 3, ale nie przez 4: {:?}", wynik);
}

---

### **Zadanie 4: Porównanie napisów**
Napisz funkcję `czy_anagramy(a: &str, b: &str) -> bool`, która sprawdzi, czy dwa napisy są anagramami (czy zawierają te same litery w innej kolejności).

fn czy_anagramy(a: &str, b: &str) -> bool {
    if a.len() != b.len() {
        return false;
    }

    let mut licznik = 0;
    let a_chars: Vec<char> = a.chars().collect();
    let b_chars: Vec<char> = b.chars().collect();

    for i in 0..a_chars.len() {
        for j in 0..b_chars.len() {
            if a_chars[i] == b_chars[j] {
                licznik += 1;
                break; // liczymy znak tylko raz
            }
        }
    }

    if licznik == a.len() {
        true
    } else {
        false
    }
}

fn main() {
    let a = "tokar";
    let b = "aktor";
    let c = "karton";

    println!("{}", czy_anagramy(a, b)); // true
    println!("{}", czy_anagramy(a, c)); // false
}

---

### **Zadanie 5: Najdłuższy wspólny prefiks**
Napisz funkcję `najdluzszy_prefiks(lista: &[String]) -> String`, która znajdzie najdłuższy wspólny początek wszystkich napisów w wektorze.

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
---