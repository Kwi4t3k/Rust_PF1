// 1

#[derive(PartialEq, Debug)]
enum Jednostka {
    Sztuki,
    Litry,
    Kilogramy,
}

#[derive(Debug)]
enum Warunki {
    Zamrazarka,
    Chlodziarka,
    Normalne,
}

#[derive(Debug)]
struct Towar {
    opis: String,
    jednostka: Jednostka,
    waga_jednostkowa_w_kilogramach: f64,
    warunki_przechowywania: Warunki,
}

impl Towar {
    fn new(opis: String, jednostka: Jednostka, mut waga_jednostkowa_w_kilogramach: f64, warunki_przechowywania: Warunki) -> Self {
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
            warunki_przechowywania
        }
    }
}

fn main() {
    let towar1 = Towar::new("Mleko".to_string(), Jednostka::Litry, 1.03, Warunki::Chlodziarka);
    let towar2 = Towar::new("Mrożona pizza".to_string(), Jednostka::Sztuki, 0.5, Warunki::Zamrazarka);
    let towar3 = Towar::new("Cukier".to_string(), Jednostka::Kilogramy, 0.8, Warunki::Normalne);

    println!("{:?}", towar1);
    println!("{:?}", towar2);
    println!("{:?}", towar3);
}

// 2