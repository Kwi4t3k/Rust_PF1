#[derive(PartialEq, Debug, Clone)]
enum Jednostka {
    Sztuki,
    Litry,
    Kilogramy,
}

#[derive(Debug, PartialEq, Clone)]
enum Warunki {
    Zamrazarka,
    Chlodziarka,
    Normalne,
}

#[derive(Debug, Clone)]
struct Towar {
    opis: String,
    jednostka: Jednostka,
    waga_jednostkowa_w_kilogramach: f64,
    warunki_przechowywania: Warunki,
}

impl Towar {
    fn new(
        opis: String,
        jednostka: Jednostka,
        mut waga_jednostkowa_w_kilogramach: f64,
        warunki_przechowywania: Warunki,
    ) -> Self {
        if waga_jednostkowa_w_kilogramach < 0.0 {
            waga_jednostkowa_w_kilogramach = 0.0;
        }

        if jednostka == Jednostka::Kilogramy {
            waga_jednostkowa_w_kilogramach = 1.0
        }

        Self {
            opis,
            jednostka,
            waga_jednostkowa_w_kilogramach,
            warunki_przechowywania,
        }
    }
}

struct Pozycja {
    towar: Towar,
    ilosc: f64,
}

struct Zamowienie {
    towary: Vec<Pozycja>,
}

impl Zamowienie {
    fn new() -> Self {
        Self { towary: Vec::new() }
    }

    fn waga_zamowienia(&self) -> f64 {
        let mut waga = 0.0;
        for pozycja in &self.towary {
            waga += pozycja.towar.waga_jednostkowa_w_kilogramach * pozycja.ilosc;
        }
        waga
    }

    fn waga_zamowienia_przechowywanie(&self, warunki: Warunki) -> f64 {
        let mut waga = 0.0;
        for pozycja in &self.towary {
            if pozycja.towar.warunki_przechowywania == warunki {
                waga += pozycja.towar.waga_jednostkowa_w_kilogramach * pozycja.ilosc;
            }
        }
        waga
    }
    fn dodaj(&mut self, towar: &Towar, mut ilosc: f64) {
        if ilosc < 0.0 {
            ilosc = 0.0;
        }

        for pozycja in &mut self.towary {
            if pozycja.towar.opis == towar.opis {
                pozycja.ilosc += ilosc;
                return;
            }
        }

        self.towary.push(
            Pozycja {
                towar: towar.clone(),
                ilosc
        })
    }
}

fn main() {
    let jablko = Towar::new("Jabłko".to_string(), Jednostka::Sztuki, 0.15, Warunki::Normalne);
    let mleko = Towar::new("Mleko".to_string(), Jednostka::Litry, 1.03, Warunki::Chlodziarka);
    let lody = Towar::new("Lody".to_string(), Jednostka::Kilogramy, 0.4, Warunki::Zamrazarka);

    // puste zamówienie
    let mut z = Zamowienie::new();

    // dodajemy pozycje
    z.dodaj(&jablko, 12.0); // 12 szt.
    z.dodaj(&mleko, 3.5); // 3,5 l
    z.dodaj(&lody, 2.0); // 2 kg
    z.dodaj(&jablko, 4.0); // +4 szt. (łącznie 16)

    println!("Całkowita waga koszyka: {:.2} kg", z.waga_zamowienia());
    println!(
        " – chłodziarka: {:.2} kg",
        z.waga_zamowienia_przechowywanie(Warunki::Chlodziarka)
    );
    println!(
        " – zamrażarka: {:.2} kg",
        z.waga_zamowienia_przechowywanie(Warunki::Zamrazarka)
    );
    println!(
        " – normalne:    {:.2} kg",
        z.waga_zamowienia_przechowywanie(Warunki::Normalne)
    );
}
