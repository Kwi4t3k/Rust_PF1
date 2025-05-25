use std::fmt;                    // importujemy fmt dla Display
use std::fmt::{Formatter};       // importujemy Formatter

// Reprezentacja koloru karty
#[derive(Copy, Clone, Debug)]
enum Kolor {
    Trefl,                      // ♣
    Karo,                       // ♦
    Kier,                       // ♥
    Pik,                        // ♠
}

// Umożliwiamy wypisywanie koloru kartą jako symbolu
impl fmt::Display for Kolor {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let symbol = match self {
            Kolor::Trefl => '♣', // przypisz znak klubu
            Kolor::Karo  => '♦', // przypisz znak karo
            Kolor::Kier  => '♥', // przypisz znak kier
            Kolor::Pik   => '♠', // przypisz znak pik
        };
        write!(f, "{}", symbol) // wypisz symbol
    }
}

// Reprezentacja wartości karty
#[derive(Copy, PartialEq, Clone, Debug)]
enum Wartosc {
    As,                         // wartość elastyczna 1/11
    Krol,                       // wartość 10
    Krolowa,                    // wartość 10
    Walet,                      // wartość 10
    Num(u8),                    // karty pipowe 2–10
}

// Wypisywanie wartości jako "A", "J", "Q", "K", "2".."10"
impl fmt::Display for Wartosc {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Wartosc::As      => "A".to_string(),
            Wartosc::Walet   => "J".to_string(),
            Wartosc::Krolowa => "Q".to_string(),
            Wartosc::Krol    => "K".to_string(),
            Wartosc::Num(n)  => n.to_string(),
        };
        write!(f, "{}", s) // wypisz oznaczenie
    }
}

impl Wartosc {
    // Podstawowy punkt: As = 11; figury = 10; Num(n) = n
    fn punkty(&self) -> u8 {
        match self {
            Wartosc::As      => 11,
            Wartosc::Krol    => 10,
            Wartosc::Krolowa => 10,
            Wartosc::Walet   => 10,
            Wartosc::Num(x)  => *x,
        }
    }
}

// Pojedyncza karta: kombinacja koloru i wartości
#[derive(Copy, Clone, Debug)]
struct Karta {
    kolor: Kolor,
    wartosc: Wartosc,
}

// Wypisywanie karty jako np. "A♠", "10♥", "K♦"
impl fmt::Display for Karta {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}{}", self.wartosc, self.kolor)
    }
}

// Talia kart (wektor 52 elementów)
struct Talia {
    talia: Vec<Karta>
}

impl Talia {
    // Konstruktor: tworzy pełną talię 52 kart, niepotasowaną
    fn new() -> Self {
        let mut talia = Vec::with_capacity(52);

        // Dla każdego koloru dodajemy As, J, Q, K, 2..10
        for &kolor in &[Kolor::Trefl, Kolor::Karo, Kolor::Kier, Kolor::Pik] {
            talia.push(Karta { kolor, wartosc: Wartosc::As });
            talia.push(Karta { kolor, wartosc: Wartosc::Walet });
            talia.push(Karta { kolor, wartosc: Wartosc::Krolowa });
            talia.push(Karta { kolor, wartosc: Wartosc::Krol });
            for x in 2..=10 {
                talia.push(Karta { kolor, wartosc: Wartosc::Num(x) });
            }
        }

        Self { talia } // zwracamy talię
    }

    // Proste tasowanie: zamienia karty według własnego pseu­do­losowego algorytmu
    fn tasuj(&mut self) {
        for i in (1..self.talia.len()).rev() {
            let j = i % (i + 1); // pseudo-losowy indeks
            self.talia.swap(i, j); // zamień karty
        }
    }

    // Rozdaj kartę z wierzchu talii (pop zwraca Option<Karta>)
    fn rozdaj(&mut self) -> Option<Karta> {
        self.talia.pop()
    }
}

// Ręka gracza lub dealera: lista kart
#[derive(Clone)]
struct Reka {
    karty: Vec<Karta>
}

impl Reka {
    fn new() -> Self {
        Self { karty: Vec::new() } // pusty wektor kart
    }

    // Dodaj kartę do ręki
    fn dobierz(&mut self, karta: Karta) {
        self.karty.push(karta);
    }

    // Oblicz sumę punktów ręki wg blackjackowych zasad As = 11 lub 1
    fn policz_punkty(&self) -> u8 {
        // 1. Sumuj wszystkie jako wartości z punkty()
        let mut suma: u8 = self.karty.iter()
            .map(|karta| karta.wartosc.punkty())
            .sum();
        // 2. Policz liczbę Asów
        let mut asy = self.karty.iter()
            .filter(|karta| karta.wartosc == Wartosc::As)
            .count();
        // 3. Dopóki suma>21 i są jeszcze Asy, odejmij 10 (As z 11→1)
        while suma > 21 && asy > 0 {
            suma -= 10;
            asy -= 1;
        }
        suma // gotowy wynik punktowy
    }
}

// Wynik jednej ręki: suma, flaga bust i blackjack
struct Wynik {
    punkty: u8,
    przebicie: bool,
    blackjack: bool,
}

impl Wynik {
    // Konstruktor z Reka
    fn z_reki(reka: Reka) -> Self {
        let punkty    = reka.policz_punkty();
        let przebicie = punkty > 21;
        let blackjack = punkty == 21 && reka.karty.len() == 2;
        Self { punkty, przebicie, blackjack }
    }
}

// Błędy, które mogą wystąpić w grze
#[derive(Debug)]
enum Blad {
    BrakKart,        // np. talia pusta
    NiepoprawnyRuch, // np. hit po bust
}

// Silnik gry: talia + ręka gracza + ręka dealera
struct Oczko {
    talia: Talia,
    reka_gracza: Reka,
    reka_dealera: Reka,
}

impl Oczko {
    // Inicjalizacja nowej gry: tworzymy i tasujemy talię, puste ręce
    fn new() -> Self {
        let mut talia = Talia::new();
        talia.tasuj();
        Oczko {
            talia,
            reka_gracza: Reka::new(),
            reka_dealera: Reka::new(),
        }
    }

    // Rozdanie startowe: każdemu po 2 karty
    fn rozdanie_startowe(&mut self) -> Result<(), Blad> {
        for _ in 0..2 {
            self.reka_gracza.dobierz(self.talia.rozdaj().ok_or(Blad::BrakKart)?);
            self.reka_dealera.dobierz(self.talia.rozdaj().ok_or(Blad::BrakKart)?);
        }
        Ok(())
    }

    // Ruch gracza: jeśli hit=true, dobierz kartę; zwróć true jeśli dalej może grać
    fn ruch_gracza(&mut self, hit: bool) -> Result<bool, Blad> {
        if hit {
            let karta = self.talia.rozdaj().ok_or(Blad::BrakKart)?;
            self.reka_gracza.dobierz(karta);
            Ok(self.reka_gracza.policz_punkty() <= 21)
        } else {
            Ok(false) // gracz pasuje
        }
    }

    // Ruch dealera: dobiera, póki <17
    fn ruch_dealera(&mut self) -> Result<(), Blad> {
        while self.reka_dealera.policz_punkty() < 17 {
            let karta = self.talia.rozdaj().ok_or(Blad::BrakKart)?;
            self.reka_dealera.dobierz(karta);
        }
        Ok(())
    }

    // Wypisz stan: karty i punkty gracza i dealera
    fn pokaz_stan(&self) {
        println!(
            "Gracz:  {} ({}) pkt",
            self.reka_gracza.karty
                .iter()
                .map(|k| k.to_string())
                .collect::<Vec<_>>()
                .join(", "),
            self.reka_gracza.policz_punkty()
        );
        println!(
            "Dealer: {} ({}) pkt",
            self.reka_dealera.karty
                .iter()
                .map(|k| k.to_string())
                .collect::<Vec<_>>()
                .join(", "),
            self.reka_dealera.policz_punkty()
        );
    }
}

// ========================
// MAIN DLA DWÓCH GRACZY
// ========================
fn main() {
    // 1) Przygotuj i potasuj talię
    let mut talia = Talia::new();
    talia.tasuj();

    // 2) Dwie oddzielne ręce
    let mut reka1 = Reka::new();
    let mut reka2 = Reka::new();

    // 3) Rozdaj każdemu po 2 karty
    for _ in 0..2 {
        reka1.dobierz(talia.rozdaj().unwrap());
        reka2.dobierz(talia.rozdaj().unwrap());
    }

    // 4) Wyświetl początkowy stan
    println!("Gracz 1: {} -> {} pkt",
        reka1.karty.iter().map(|k| k.to_string()).collect::<Vec<_>>().join(", "),
        reka1.policz_punkty()
    );
    println!("Gracz 2: {} -> {} pkt",
        reka2.karty.iter().map(|k| k.to_string()).collect::<Vec<_>>().join(", "),
        reka2.policz_punkty()
    );

    // 5) Prosta strategia: każdy dobiera do 17
    while reka1.policz_punkty() < 17 {
        reka1.dobierz(talia.rozdaj().unwrap());
    }
    while reka2.policz_punkty() < 17 {
        reka2.dobierz(talia.rozdaj().unwrap());
    }

    // 6) Oblicz wyniki
    let w1 = Wynik::z_reki(reka1.clone());
    let w2 = Wynik::z_reki(reka2.clone());

    // 7) Wyświetl ostateczne ręce i wyniki
    println!(
        "\nGracz 1 końcowo: {} -> {} pkt (bust={}, BJ={})",
        reka1.karty.iter().map(|k| k.to_string()).collect::<Vec<_>>().join(", "),
        w1.punkty, w1.przebicie, w1.blackjack
    );
    println!(
        "Gracz 2 końcowo: {} -> {} pkt (bust={}, BJ={})",
        reka2.karty.iter().map(|k| k.to_string()).collect::<Vec<_>>().join(", "),
        w2.punkty, w2.przebicie, w2.blackjack
    );

    // 8) Porównanie zwycięzcy
    match (w1.przebicie, w2.przebicie) {
        (true, true)   => println!("Obaj przegrali (bust)."),
        (true, false)  => println!("Gracz 2 wygrał, gracz 1 bust."),
        (false, true)  => println!("Gracz 1 wygrał, gracz 2 bust."),
        (false, false) => {
            if w1.punkty > w2.punkty {
                println!("Gracz 1 wygrał na punkty.");
            } else if w2.punkty > w1.punkty {
                println!("Gracz 2 wygrał na punkty.");
            } else {
                println!("Remis punktowy.");
            }
        }
    }
}

// fn main() {
//     // ===== WERSJA DLA JEDNEGO GRACZA =====
//     // 1) Przygotuj nową grę (talia, ręka gracza, ręka dealera)
//     let mut gra = Oczko::new();
//     // 2) Rozdaj po dwie karty (gracz + dealer)
//     gra.rozdanie_startowe().unwrap();
//     // 3) Pokaż stan po rozdaniu początkowym
//     println!("--- Rozdanie startowe ---");
//     gra.pokaz_stan();
//
//     // 4) Symulujemy prostą logikę „hit until stand”:
//     //    gracz dobiera do 17 lub do bust
//     while gra.reka_gracza.policz_punkty() < 17 {
//         let cont = gra.ruch_gracza(true).unwrap();
//         if !cont { break } // jeśli już busted albo stand, wychodzimy
//     }
//
//     // 5) Dealer dobiera wg. zasad (do 17)
//     gra.ruch_dealera().unwrap();
//
//     // 6) Wyświetlamy ostateczny wynik
//     println!("\n--- Wynik końcowy ---");
//     gra.pokaz_stan();
// }