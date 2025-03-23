// 1. Napisz funkcję o nagłówku
// fn co_drugi_znak(napis: ...) -> ...
//
// która zwróci napis zawierający co drugi znak z danego napisu.
//

fn co_drugi_znak(napis: &str) -> String {
    let mut nowy_napis = String::new(); // nowa tablica dynamiczna Stringów

    for c in napis.chars().step_by(2) { // pętla po znakach z napisu co 2 elementy
        nowy_napis.push(c); // dodawanie elementów do nowej tablicy
        // nowy_napis.push(' '); spacja do stringa
    }
    nowy_napis // zwracanie nowej tablicy
}

fn co_drugi_znak_v2(napis: &str) -> String {
    // collect konsumuje iterator i przekształca w odpowiednią kolekcję
    napis.chars().step_by(2).collect()
}

fn main() {
    println!("{}", co_drugi_znak("napis")); // n p s
    println!("{}", co_drugi_znak_v2("napis"));
}

// 2. Zdefiniuj funkcję o nagłówku
// fn szyfruj(napis: ..., klucz: ...) -> ...
//
// która dla danego napisu zwróci ten sam napis zaszyfrowany prostym szyfrem odwracającym — klucz określa długość odwracanych fragmentów. Przykłady:
//
// szyfruj("Aladyn", 2) == "lAdany"
// szyfruj("Aladyn", 3) == "alAnyd"
// szyfruj("Aladyn", 4) == "dalAny"
// szyfruj("Aladyn", 5) = "ydalAn"
// szyfruj("koza", 3) == "zoka"
// szyfruj("kaszanka", 3) == "saknazak"
// szyfruj("kot Mruczek", 9) == "zcurM tokke"
// szyfruj("kot Mruczek", 1) == "kot Mruczek"
// szyfruj("kot Mruczek", 2) == "ok trMcuezk"
//

// Funkcja, która odwraca tylko pierwsze 'klucz' znaków, resztę zostawia bez zmian
fn szyfruj_ile_znakow_od_przodu(napis: &str, klucz: usize) -> String {
    let mut zaszyfrowany_napis = String::new(); // Tworzymy pusty napis na wynik
    let wycinek_lewy = &napis[0..klucz]; // Bierzemy pierwszy fragment o długości 'klucz'
    let wycinek_prawy = &napis[klucz..]; // Bierzemy resztę napisu od klucza do końca

    for i in wycinek_lewy.chars().rev() { // Odwracamy pierwszy fragment znaków
        zaszyfrowany_napis.push(i); // Dodajemy odwrócone znaki do wyniku
    }
    for i in wycinek_prawy.chars() { // Dodajemy resztę znaków bez zmian
        zaszyfrowany_napis.push(i);
    }

    // zwracamy zaszyfrowany napis
    zaszyfrowany_napis
}

// Funkcja, która dzieli napis na fragmenty o długości klucz i odwraca je
fn szyfruj(napis: &str, klucz: usize) -> String {
    let mut zakodowane = String::new(); // Tworzymy pusty napis na wynik

    for i in (0..napis.len()).step_by(klucz) { // Iterujemy po napisie co 'klucz' bajtów
        if i+klucz < napis.len() { // Jeżeli zostało co najmniej 'klucz' bajtów
            // Bierzemy kawałek długości 'klucz', odwracamy go i dodajemy do wyniku
            zakodowane += napis[i..i+klucz].chars().rev().collect::<String>().as_str();
        } else {
            // Jeśli jesteśmy na końcu i zostało mniej znaków niż 'klucz', odwracamy resztę
            zakodowane += napis[i..].chars().rev().collect::<String>().as_str();
        }
    }

    zakodowane // Zwracamy zakodowany napis
}

// Inna wersja szyfrowania, używająca wektora znaków i podziału na kawałki
fn szyfruj(napis: &str, klucz: usize) -> String {
    let mut zaszyfrowany_napis = String::new(); // Tworzymy pusty napis na wynik
    let znaki: Vec<char> = napis.chars().collect(); // Zamieniamy napis na wektor znaków

    // Przechodzimy przez napis w kawałkach długości 'klucz'
    for fragment in znaki.chunks(klucz) {
        // Odwracamy każdy fragment
        for c in fragment.iter().rev() {
            zaszyfrowany_napis.push(*c); // Dodajemy odwrócone znaki do wyniku
        }
    }

    zaszyfrowany_napis // Zwracamy zaszyfrowany napis
}

// Kolejna wersja szyfrowania z buforem na znaki
fn szyfruj(napis: &str, klucz: usize) -> String {
    let mut zaszyfrowany_napis = String::new(); // Tworzymy pusty napis na wynik
    let mut bufor = Vec::new(); // Tworzymy bufor na fragment o długości klucz

    for c in napis.chars() { // Iterujemy po wszystkich znakach napisu
        bufor.push(c); // Dodajemy znak do bufora

        // Jeżeli bufor osiągnął długość 'klucz'
        if bufor.len() >= klucz {
            // Odwracamy bufor i dodajemy do wyniku
            while let Some(znak) = bufor.pop() {
                zaszyfrowany_napis.push(znak); // Dodajemy odwrócone znaki do wyniku
            }
        }
    }

    // Jeśli zostały znaki w buforze (ostatni fragment krótszy niż klucz)
    while let Some(znak) = bufor.pop() {
        zaszyfrowany_napis.push(znak); // Dodajemy pozostałe znaki, odwrócone
    }

    zaszyfrowany_napis // Zwracamy zaszyfrowany napis
}

// Funkcja main z przykładami testowymi
fn main() {
    println!("{}", szyfruj("Aladyn", 2)); // lAdany
    println!("{}", szyfruj("Aladyn", 3)); // alAnyd
    println!("{}", szyfruj("Aladyn", 4)); // dalAny
    println!("{}", szyfruj("Aladyn", 5)); // ydalAn
    println!("{}", szyfruj("koza", 3));   // zoka
    println!("{}", szyfruj("kaszanka", 3)); // saknazak
    println!("{}", szyfruj("kot Mruczek", 9)); // zcurM tokke
    println!("{}", szyfruj("kot Mruczek", 1)); // kot Mruczek
    println!("{}", szyfruj("kot Mruczek", 2)); // ok trMcuezk
}

// 3. Napisz funkcję wizytowka, która otrzymuje w dwóch parametrach napisowych imię i nazwisko, a zwraca napis powstały z pierwszej litery imienia, kropki, spacji i nazwiska, przy czym w wyniku pierwsza litera imienia i nazwiska mają być duże, pozostałe małe. Na przykład, dla danych "jan" oraz "KOWALSKI" funkcja ma zwracać napis "J. Kowalski".
//

fn wizytowka(imie: &str, nazwisko: &str) -> String {
    let mut wynik = String::new(); // Tworzymy pusty napis, do którego będziemy dodawać wynik

    let male_imie = imie.to_lowercase(); // Zamieniamy imię na małe litery (np. "JAN" → "jan")
    let male_nazwisko = nazwisko.to_lowercase(); // Zamieniamy nazwisko na małe litery (np. "KOWALSKI" → "kowalski")

    // Bierzemy pierwszą literę imienia i zamieniamy ją na dużą literę
    let pierwsza_litera_imienia = male_imie
        .chars() // Tworzymy iterator po znakach imienia
        .next() // Bierzemy pierwszy znak
        .unwrap() // Rozpakowujemy wartość (bo next() zwraca Option)
        .to_uppercase(); // Zamieniamy na dużą literę (może zwrócić więcej niż 1 znak)

    wynik.push_str(&pierwsza_litera_imienia.to_string()); // Dodajemy dużą literę imienia do wyniku
    wynik.push_str(". "); // Dodajemy kropkę i spację po inicjale imienia

    let mut litery_nazwiska = male_nazwisko.chars(); // Tworzymy iterator po znakach nazwiska

    // Sprawdzamy, czy nazwisko ma co najmniej jeden znak
    if let Some(pierwsza_litera) = litery_nazwiska.next() {
        // Dodajemy pierwszą literę nazwiska jako dużą literę
        wynik.push_str(&pierwsza_litera.to_uppercase().to_string());

        // Dodajemy pozostałe litery nazwiska (małe litery)
        for litera in litery_nazwiska {
            wynik.push(litera);
        }
    }

    wynik // Zwracamy gotowy wynik
}

fn wizytowka(imie: &str, nazwisko: &str) -> String {
    // Zamieniamy imię na małe litery
    let imie_male = imie.to_lowercase(); // np. "JAN" → "jan"

    // Zamieniamy nazwisko na małe litery
    let nazwisko_male = nazwisko.to_lowercase(); // np. "KOWALSKI" → "kowalski"

    // Bierzemy pierwszą literę imienia, robimy dużą
    let pierwsza_litera_imienia = imie_male
        .chars() // zamieniamy napis na iterator znaków
        .next() // bierzemy pierwszy znak
        .unwrap() // rozpakowujemy (bo next() daje Option)
        .to_uppercase() // zamieniamy na dużą literę (może zwrócić więcej niż 1 znak np. ß)
        .to_string(); // zamieniamy na String

    // Bierzemy pierwszą literę nazwiska, robimy dużą
    let pierwsza_litera_nazwiska = nazwisko_male
        .chars() // zamieniamy napis na iterator znaków
        .next() // bierzemy pierwszy znak
        .unwrap() // rozpakowujemy Option
        .to_uppercase() // robimy dużą literę
        .to_string(); // zamieniamy na String

    // Reszta nazwiska (od drugiej litery)
    let reszta_nazwiska = &nazwisko_male[1..]; // bierzemy slice od indeksu 1 do końca

    // Sklejamy wszystko razem
    let wynik = pierwsza_litera_imienia // dodajemy pierwszą literę imienia (duża)
        + ". " // dodajemy kropkę i spację
        + &pierwsza_litera_nazwiska // dodajemy pierwszą literę nazwiska (duża)
        + reszta_nazwiska; // dodajemy resztę nazwiska (małe litery)

    wynik // zwracamy gotowy napis
}

fn main() {
    println!("{}", wizytowka("jan", "KOWALSKI")); // J. Kowalski
    println!("{}", wizytowka("ALICJA", "nowak")); // A. Nowak
}

// 4. Napisz funkcję o nagłówku
// fn na_rzymskie(liczba: ...) -> ...
//
// która dla danej liczby całkowitej zwraca jej zapis rzymski. Przykłady:
// na_rzymskie(3) == "III"
// na_rzymskie(9) == "IX"
// na_rzymskie(19) == "XIX"
// na_rzymskie(1910) == "MCMX"
//

fn na_rzymskie(mut liczba: u32) -> String { // Funkcja przyjmuje liczbę całkowitą i zwraca String
    let mut wynik = String::new(); // Tworzymy pusty napis, do którego będziemy dodawać symbole rzymskie

    let rzymskie = [ // Tablica krotek - każda zawiera wartość liczbową i odpowiadający jej symbol rzymski
        (1000, "M"),
        (500, "D"),
        (100, "C"),
        (50, "L"),
        (10, "X"),
        (5, "V"),
        (1, "I"),
    ];

    // Iterujemy po każdej krotce (wartość, symbol) w tablicy od największej wartości do najmniejszej
    for &(wartosc, symbol) in rzymskie.iter() {
        // Dopóki nasza liczba jest większa lub równa wartości symbolu
        while liczba >= wartosc {
            wynik.push_str(symbol); // Dodajemy symbol do wyniku
            liczba -= wartosc; // Odejmuje wartość symbolu od liczby
        }
    }

    wynik // Zwracamy napis złożony z symboli rzymskich
}

fn main() {
    println!("{}", na_rzymskie(3));    // Wywołanie funkcji dla liczby 3 -> wynik: III
    println!("{}", na_rzymskie(9));    // Wywołanie funkcji dla liczby 9 -> wynik: VIIII (bez skrótu IX)
    println!("{}", na_rzymskie(19));   // Wywołanie funkcji dla liczby 19 -> wynik: XVIIIIII
    println!("{}", na_rzymskie(1910)); // Wywołanie funkcji dla liczby 1910 -> wynik: MDCCCCX
}

// 5. Napisz funkcję o nagłówku
// fn dodaj_pisemnie(a: ..., b: ...) -> ...
//
// która doda dwie (zakładamy, że poprawne) liczby całkowite podane w argumentach jako napisy w zapisie dziesiętnym — i zwróci wynik również jako napis. Uwaga: dodawanie należy przeprowadzić pisemnie, bowiem liczby mogą być dowolnie duże. Przykłady:
//
// dodaj_pismnie("1", "3") == "4"
// dodaj_pismnie("1", "3") == "4"
// dodaj_pismnie("8", "3") == "11"
// dodaj_pismnie("10", "23") == "33"
// dodaj_pismnie("1", "0") == "1"
// dodaj_pismnie("11", "00") == "11"
// dodaj_pismnie("131", "9900") == "10031"
// dodaj_pismnie("998", "7") == "1005"
// dodaj_pismnie("24872947", "294729478") == "319602425"
// dodaj_pismnie("5924729874298749827418582", "6782893629472094209740298") == "12707623503770844037158880"
//

fn dodaj_pisemnie(a: &str, b: &str) -> String {
    let mut wynik = String::new(); // Tutaj będziemy budować wynik dodawania jako napis
    let mut przeniesienie = 0;     // Zmienna do przechowywania przeniesienia (np. 1 gdy suma > 9)

    // Zamieniamy napisy na wektory znaków, żeby łatwo móc odwoływać się do każdej cyfry
    let a_chars: Vec<char> = a.chars().collect();
    let b_chars: Vec<char> = b.chars().collect();

    // Pobieramy indeksy ostatnich cyfr (bo dodajemy od końca jak pisemnie)
    let mut i = a_chars.len() as i32 - 1;
    let mut j = b_chars.len() as i32 - 1;

    // Pętla trwa dopóki są cyfry do dodania lub jest przeniesienie
    while i >= 0 || j >= 0 || przeniesienie > 0 {
        let mut suma = przeniesienie; // Zaczynamy od przeniesienia (na początku 0)

        // Jeśli są jeszcze cyfry w liczbie a, dodajemy je do sumy
        if i >= 0 {
            // Zamieniamy znak na cyfrę
            suma += a_chars[i as usize].to_digit(10).unwrap();
            i -= 1; // Przechodzimy do poprzedniej cyfry
        }

        // Jeśli są jeszcze cyfry w liczbie b, dodajemy je do sumy
        if j >= 0 {
            suma += b_chars[j as usize].to_digit(10).unwrap();
            j -= 1; // Przechodzimy do poprzedniej cyfry
        }

        // Obliczamy nowe przeniesienie (np. 15 -> przeniesienie 1)
        przeniesienie = suma / 10;

        // Dodajemy ostatnią cyfrę do wyniku (np. z 15 bierzemy 5)
        let cyfra = (suma % 10).to_string();
        wynik = cyfra + &wynik; // Dodajemy cyfrę na początek wyniku
    }

    wynik // Zwracamy końcowy napis z wynikiem
}

fn main() {
    println!("{}", dodaj_pisemnie("1", "3")); // 4
    println!("{}", dodaj_pisemnie("8", "3")); // 11
    println!("{}", dodaj_pisemnie("10", "23")); // 33
    println!("{}", dodaj_pisemnie("1", "0")); // 1
    println!("{}", dodaj_pisemnie("11", "00")); // 11
    println!("{}", dodaj_pisemnie("131", "9900")); // 10031
    println!("{}", dodaj_pisemnie("998", "7")); // 1005
    println!("{}", dodaj_pisemnie("24872947", "294729478")); // 319602425
    println!("{}", dodaj_pisemnie("5924729874298749827418582", "6782893629472094209740298")); // 12707623503770844037158880
}
