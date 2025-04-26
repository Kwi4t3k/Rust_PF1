// 1
#[derive(Debug)]
enum Jednostka {
    Sztuki,
    Litry,
    Kilogramy,
}

#[derive(Debug)]
enum Przechowywacz {
    Zamrazarka,
    Chlodziarka,
    Normalne,
}

#[derive(Debug)]
struct Towar {
    nazwa: String,
    jednostka: Jednostka,
    waga_jednostkowa_w_kilogramach: f64,
    wymagagane_warunki_przechowywania: Przechowywacz,
}

impl Towar {
    fn new(nazwa: String, jednostka: Jednostka, mut waga_jednostkowa_w_kilogramach: f64, wymagagane_warunki_przechowywania: Przechowywacz) -> Option<Self> {
        if waga_jednostkowa_w_kilogramach <= 0.0 {
            return None;
        }

        if let Jednostka::Kilogramy = jednostka {
            waga_jednostkowa_w_kilogramach = 1.0;
        }

        Some(
            Self {
                nazwa,
                jednostka,
                waga_jednostkowa_w_kilogramach,
                wymagagane_warunki_przechowywania
            }
        )
    }
}

fn main() {
    let towar1 = Towar::new("Mleko".to_string(), Jednostka::Litry, 1.03, Przechowywacz::Chlodziarka);
    let towar2 = Towar::new("Mrożona pizza".to_string(), Jednostka::Sztuki, 0.5, Przechowywacz::Zamrazarka);
    let towar3 = Towar::new("Cukier".to_string(), Jednostka::Kilogramy, 0.8, Przechowywacz::Normalne);

    println!("{:?}", towar1);
    println!("{:?}", towar2);
    println!("{:?}", towar3);
}

// 2

#[derive(Debug, Clone, Copy, PartialEq)]
enum Jednostka { Sztuki, Litry, Kilogramy }

#[derive(Debug, Clone, Copy, PartialEq)]
enum Przechowywacz { Zamrazarka, Chlodziarka, Normalne }

#[derive(Debug, Clone)]
struct Towar {
    nazwa: String,
    jednostka: Jednostka,
    waga_jednostkowa_w_kg: f64,
    warunki: Przechowywacz,
}

impl Towar {
    fn new(
        nazwa: &str,
        jednostka: Jednostka,
        mut waga_jednostkowa_w_kg: f64,
        warunki: Przechowywacz,
    ) -> Option<Self> {
        if waga_jednostkowa_w_kg <= 0.0 { return None }      // waga > 0
        if let Jednostka::Kilogramy = jednostka {             // dla „kg” → 1.0
            waga_jednostkowa_w_kg = 1.0;
        }
        Some(Self {
            nazwa: nazwa.to_string(),
            jednostka,
            waga_jednostkowa_w_kg,
            warunki,
        })
    }
}

/* ────────────────────────────────────────────────
   2.  POZYCJA w zamówieniu  =  (Towar, ilość)
   ────────────────────────────────────────────── */

#[derive(Debug, Clone)]
struct Pozycja { towar: Towar, ilosc: f64 }

/* ────────────────────────────────────────────────
   3.  Zamówienie  –  trzymamy pozycje w zwykłym Vec-ie
   ────────────────────────────────────────────── */

#[derive(Debug, Default)]
struct Zamowienie { pozycje: Vec<Pozycja> }

impl Zamowienie {
    /// Pusty koszyk
    fn new() -> Self { Self { pozycje: Vec::new() } }

    /// Dokładamy towar
    fn dodaj(&mut self, towar: Towar, ilosc: f64) -> Result<(), &'static str> {
        // ilość dodatnia
        if ilosc <= 0.0 { return Err("ilość musi być dodatnia") }
        // dla sztuk – ilość całkowita
        if matches!(towar.jednostka, Jednostka::Sztuki) && ilosc.fract() != 0.0 {
            return Err("dla sztuk ilość musi być całkowita")
        }

        /*  jeśli taki sam towar już jest w wektorze,
            to tylko zwiększamy ilość                           */
        for poz in &mut self.pozycje {
            if  poz.towar.nazwa     == towar.nazwa &&
                poz.towar.jednostka == towar.jednostka &&
                (poz.towar.waga_jednostkowa_w_kg - towar.waga_jednostkowa_w_kg).abs() < 1e-9 &&
                poz.towar.warunki   == towar.warunki
            {
                poz.ilosc += ilosc;
                return Ok(())
            }
        }

        // inaczej – dopisujemy nową pozycję
        self.pozycje.push(Pozycja{ towar, ilosc });
        Ok(())
    }

    /// Suma wag wszystkich pozycji (kg)
    fn waga_calkowita(&self) -> f64 {
        let mut suma = 0.0;
        for p in &self.pozycje {
            suma += p.towar.waga_jednostkowa_w_kg * p.ilosc;
        }
        suma
    }

    /// Suma wag pozycji wymagających podanych warunków
    fn waga_dla(&self, war: Przechowywacz) -> f64 {
        let mut suma = 0.0;
        for p in &self.pozycje {
            if p.towar.warunki == war {
                suma += p.towar.waga_jednostkowa_w_kg * p.ilosc;
            }
        }
        suma
    }
}

fn main() {
    // tworzymy trzy poprawne towary
    let jablko = Towar::new("Jabłko", Jednostka::Sztuki, 0.15, Przechowywacz::Normalne).unwrap();
    let mleko  = Towar::new("Mleko",  Jednostka::Litry,  1.03, Przechowywacz::Chlodziarka).unwrap();
    let lody   = Towar::new("Lody",   Jednostka::Kilogramy, 0.4, Przechowywacz::Zamrazarka).unwrap();

    // puste zamówienie
    let mut z = Zamowienie::new();

    // dodajemy pozycje
    z.dodaj(jablko.clone(), 12.0).unwrap();  // 12 szt.
    z.dodaj(mleko.clone(),  3.5 ).unwrap();  // 3,5 l
    z.dodaj(lody.clone(),   2.0 ).unwrap();  // 2 kg
    z.dodaj(jablko,         4.0 ).unwrap();  // +4 szt. (łącznie 16)

    println!("Całkowita waga koszyka: {:.2} kg", z.waga_calkowita());
    println!(" – chłodziarka: {:.2} kg", z.waga_dla(Przechowywacz::Chlodziarka));
    println!(" – zamrażarka: {:.2} kg",  z.waga_dla(Przechowywacz::Zamrazarka));
    println!(" – normalne:    {:.2} kg", z.waga_dla(Przechowywacz::Normalne));
}