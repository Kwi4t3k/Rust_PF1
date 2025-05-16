
// Tradycyjne karty do gry (brydż, poker itd.) podzielone są na kolory o nazwach:

// pik,
// kier,
// karo,
// trefl.

// Zaprojektuj typ wyliczeniowy, który będzie mógł reprezentować dane o kolorze z dodatkowym warunkiem, że są one uporządkowane jak w brydżu (zgodnie z wypunktowaniem powyżej, gdzie są podane malejąco). Przetestuj wszystko.

// Wskazówka: Derywuj odpowiednią cechę i ułóż warianty w odpowiedniej kolejności.

#[derive(PartialOrd, Ord, PartialEq, Eq, Debug)]

enum Kolor {
    Pik,
    Kier,
    Karo,
    Trefl,
}

fn main() {
    // Przykładowa lista kolorów w losowej kolejności
    let mut kolory = vec![Kolor::Trefl, Kolor::Kier, Kolor::Pik, Kolor::Karo];

    // Sortowanie zgodnie z kolejnością brydżową
    kolory.sort();

    // Wyświetlenie wyników
    for kolor in &kolory {
        println!("{:?}", kolor);
    }
}


// Zaprojektuj typ do przechwoywania informacji o następujących możliwych błędach:

// brak błędu;
// zły format pliku;
// plik nie istnieje (z tym błędem musi być związana wartość nazwy pliku);
// plik zbyt duży (z tym błędem muszą być związane dwie wartości — rozmiar aktualny i rozmiar maksymalny).

// Dodatkowo zaimplementuj dla tego typu metodę o nazwie pokaz_komunikat, która wyświetli pełny komunikat o podanym błędzie (wraz z jego danymi).

#[derive(Debug)]

enum Blad {
    BrakBledu,
    ZlyFormatPliku,
    PlikNieIstnieje(String),
    PlikZbytDuzy(u64, u64),
}

impl Blad {
    fn pokaz_komunikat(&self) {
        match self {
            Blad::BrakBledu => println!("Brak błędu"),
            Blad::ZlyFormatPliku => println!("Zły format pliku"),
            Blad::PlikNieIstnieje(nazwa) => println!("Plik {} nie istnieje", nazwa),
            Blad::PlikZbytDuzy(rozmiar, max_rozmiar) => println!("Plik zbyt duży, rozmiar pliku to {}, maksymalny rozmiar pliku to {}", rozmiar, max_rozmiar),
        }
    }
}

fn main() {
    let bledy = vec![
        Blad::BrakBledu,
        Blad::ZlyFormatPliku,
        Blad::PlikNieIstnieje(String::from("dokument.txt")),
        Blad::PlikZbytDuzy(10_000, 5_000),
    ];

    for blad in &bledy {
        blad.pokaz_komunikat();
    }
}