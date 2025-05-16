#[derive(Debug, Clone, Copy)]
enum Kolor {
    Pik,
    Trefl,
    Karo,
    Kier,
}

#[derive(Debug, Clone, Copy)]
enum Wartosc {
    As,
    Dwa,
    Trzy,
    Cztery,
    Piec,
    Szesc,
    Siedem,
    Osiem,
    Dziewiec,
    Dziesiec,
    Walet,
    Dama,
    Krol,
}

impl Wartosc {
    fn wartosc_punktowa(&self) -> u32 {
        match self {
            Wartosc::As => 1,
            Wartosc::Dwa => 2,
            Wartosc::Trzy => 3,
            Wartosc::Cztery => 4,
            Wartosc::Piec => 5,
            Wartosc::Szesc => 6,
            Wartosc::Siedem => 7,
            Wartosc::Osiem => 8,
            Wartosc::Dziewiec => 9,
            Wartosc::Dziesiec | Wartosc::Walet | Wartosc::Dama | Wartosc::Krol => 10,
        }
    }
}

#[derive(Debug, Clone)]
struct Karta {
    kolor: Kolor,
    wartosc: Wartosc,
}

struct Talia {
    karty: Vec<Karta>,
}

impl Talia {
    fn nowa() -> Self {
        let mut karty = Vec::new();
        let kolory = vec![Kolor::Kier, Kolor::Karo, Kolor::Pik, Kolor::Trefl];
        let wartosci = vec![
            Wartosc::As, Wartosc::Dwa, Wartosc::Trzy, Wartosc::Cztery,
            Wartosc::Piec, Wartosc::Szesc, Wartosc::Siedem, Wartosc::Osiem,
            Wartosc::Dziewiec, Wartosc::Dziesiec, Wartosc::Walet, Wartosc::Dama,
            Wartosc::Krol,
        ];

        for kolor in &kolory {
            for wartosc in &wartosci {
                karty.push(Karta { kolor: *kolor, wartosc: *wartosc });
            }
        }

        Self { karty }
    }

    fn losuj_liczbe(seed: &mut u32) -> usize {
        *seed = (*seed).wrapping_mul(1664525).wrapping_add(1013904223);
        ((*seed >> 16) & 51) as usize
    }

    fn tasuj(&mut self, seed: u32) {
        let mut seed = seed;
        let len = self.karty.len();
        for i in (0..len).rev() {
            let j = Self::losuj_liczbe(&mut seed) % (i + 1);
            self.karty.swap(i, j);
        }
    }

    fn dobierz_karte(&mut self) -> Option<Karta> {
        self.karty.pop()
    }
}

struct Reka {
    karty: Vec<Karta>,
    punkty: u32,
}

impl Reka {
    fn nowa() -> Self {
        Self { karty: Vec::new(), punkty: 0 }
    }

    fn dodaj_karte(&mut self, karta: Karta) {
        self.punkty += karta.wartosc as u32;
        self.karty.push(karta);
    }

    fn przekroczono_limit(&self) -> bool {
        self.punkty > 21
    }
}

enum WynikGry {
    Wygrana,
    Przegrana,
    Kontynuuj,
}

fn main() {
    let mut talia = Talia::nowa();
    let seed = 123456;
    talia.tasuj(seed);

    let mut reka = Reka::nowa();

    while reka.punkty < 21 {
        if let Some(karta) = talia.dobierz_karte() {
            reka.dodaj_karte(karta.clone());
            println!("Dobierasz kartę: {:?}", karta);
            println!("Łączna liczba punktów: {}", reka.punkty);

            if reka.przekroczono_limit() {
                println!("Przegrałeś!");
                break;
            }
        }
    }

    println!("Koniec gry, zdobyłeś {} punktów!", reka.punkty);
}