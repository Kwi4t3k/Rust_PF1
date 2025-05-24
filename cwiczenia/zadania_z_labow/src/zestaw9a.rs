// 1

#[derive(Debug, PartialEq, PartialOrd)]
enum Kolor {
    Trefl, // 0
    Karo,  // 1
    Kier,  // 2
    Pik,   // 3
}

fn main() {
    let pik = Kolor::Pik;
    let kier = Kolor::Kier;
    let karo = Kolor::Karo;
    let trefl = Kolor::Trefl;

    println!("{}", pik > kier);
    println!("{}", kier > karo);
    println!("{}", karo > trefl);
}

// 2

enum Error {
    Brak,
    ZlyFormatPliku,
    PlikNieIstnieje {
        nazwa_pliku: String,
    },
    PlikZbytDuzy {
        rozmiar_aktualny: u64,
        rozmiar_maksymalny: u64,
    },
}

impl Error {
    fn pokaz_komunikat(&self) {
        match self {
            Error::Brak => println!("Brak błędu"),
            Error::ZlyFormatPliku => println!("Zły format pliku"),
            Error::PlikNieIstnieje { nazwa_pliku } => println!("Plik {nazwa_pliku} nie istnieje"),
            Error::PlikZbytDuzy {
                rozmiar_aktualny,
                rozmiar_maksymalny,
            } => println!(
                "Maksymalny rozmiar pliku to {rozmiar_maksymalny}, a twój rozmiar pliku to {rozmiar_aktualny}"
            ),
        }
    }
}

fn main() {
    let no_error = Error::Brak;
    no_error.pokaz_komunikat();
    let invalid_format = Error::ZlyFormatPliku;
    invalid_format.pokaz_komunikat();
    let not_found = Error::PlikNieIstnieje {
        nazwa_pliku: "test.file".to_string(),
    };
    not_found.pokaz_komunikat();
    let too_large = Error::PlikZbytDuzy {
        rozmiar_aktualny: 121,
        rozmiar_maksymalny: 100,
    };
    too_large.pokaz_komunikat();
}