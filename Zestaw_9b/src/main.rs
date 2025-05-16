// Definicja enumu Miesiac - reprezentuje 12 miesiecy roku
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum Miesiac {
    Styczen, Luty, Marzec, Kwiecien, Maj, Czerwiec,
    Lipiec, Sierpien, Wrzesien, Pazdziernik, Listopad, Grudzien,
}

impl Miesiac {
    // Funkcja zwracajaca liczbe odpowiadajaca danemu miesiacowi
    fn numer(self) -> u8 {
        match self {
            Miesiac::Styczen => 1,
            Miesiac::Luty => 2,
            Miesiac::Marzec => 3,
            Miesiac::Kwiecien => 4,
            Miesiac::Maj => 5,
            Miesiac::Czerwiec => 6,
            Miesiac::Lipiec => 7,
            Miesiac::Sierpien => 8,
            Miesiac::Wrzesien => 9,
            Miesiac::Pazdziernik => 10,
            Miesiac::Listopad => 11,
            Miesiac::Grudzien => 12,
        }
    }
}

// Struktura Czas przechowuje godzine, minute i sekunde
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
struct Czas {
    godzina: u8,   // Pole godzina (0–23)
    minuta: u8,    // Pole minuta (0–59)
    sekunda: u8,   // Pole sekunda (0–59)
}

impl Czas {
    // Tworzy nowy Czas z podanych trzech wartosci
    fn z_3(godzina: u8, minuta: u8, sekunda: u8) -> Czas {
        Czas { godzina, minuta, sekunda }
    }

    // Tworzy Czas z lancucha znakow rozdzielonego znakiem (np. "12-34-56")
    fn z_napisu(napis: &str, znak: char) -> Czas {
        let czesci: Vec<&str> = napis.split(znak).collect(); // Dzieli tekst na czesci wg separatora
        let h = czesci.get(0).and_then(|s| s.parse().ok()).unwrap_or(0); // Parsuje godzine lub 0
        let m = czesci.get(1).and_then(|s| s.parse().ok()).unwrap_or(0); // Parsuje minute lub 0
        let s = czesci.get(2).and_then(|s| s.parse().ok()).unwrap_or(0); // Parsuje sekunde lub 0
        Czas::z_3(h, m, s) // Tworzy nowy obiekt Czas
    }

    // Zwraca tekst w formacie "HH:MM:SS"
    fn na_napis(&self) -> String {
        format!("{:02}:{:02}:{:02}", self.godzina, self.minuta, self.sekunda)
    }
}

// Struktura Data przechowuje dzien, miesiac, rok i opcjonalnie czas
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
struct Data {
    rok: u16,              // Pole rok (np. 2025)
    miesiac: Miesiac,      // Pole miesiac jako enum
    dzien: u8,             // Pole dzien (1–31)
    czas: Option<Czas>,    // Opcjonalny czas
}

impl Data {
    // Tworzy nowa date bez czasu
    fn z_3(dzien: u8, miesiac: Miesiac, rok: u16) -> Data {
        Data { rok, miesiac, dzien, czas: None }
    }

    // Tworzy date z lancucha znakow "DD<znak>MM<znak>YYYY"
    fn z_napisu(napis: &str, znak: char) -> Data {
        let czesci: Vec<&str> = napis.split(znak).collect(); // Dzieli tekst na czesci
        let d = czesci.get(0).and_then(|s| s.parse().ok()).unwrap_or(1); // Parsuje dzien
        let m_nr = czesci.get(1).and_then(|s| s.parse().ok()).unwrap_or(1); // Parsuje miesiac jako numer
        let r = czesci.get(2).and_then(|s| s.parse().ok()).unwrap_or(1970); // Parsuje rok

        // Konwersja numeru miesiaca na enum Miesiac
        let m = match m_nr {
            1 => Miesiac::Styczen,
            2 => Miesiac::Luty,
            3 => Miesiac::Marzec,
            4 => Miesiac::Kwiecien,
            5 => Miesiac::Maj,
            6 => Miesiac::Czerwiec,
            7 => Miesiac::Lipiec,
            8 => Miesiac::Sierpien,
            9 => Miesiac::Wrzesien,
            10 => Miesiac::Pazdziernik,
            11 => Miesiac::Listopad,
            12 => Miesiac::Grudzien,
            _ => Miesiac::Styczen,
        };

        Data::z_3(d, m, r) // Zwraca nowa date
    }

    // Zwraca tekst "DD.MM.RRRR" lub "DD.MM.RRRR HH:MM:SS"
    fn na_napis(&self) -> String {
        let data = format!("{:02}.{:02}.{:04}", self.dzien, self.miesiac.numer(), self.rok); // Format daty
        if let Some(c) = &self.czas {
            format!("{} {}", data, c.na_napis()) // Jesli jest czas, dodaje do daty
        } else {
            data // Sama data bez czasu
        }
    }

    // Sprawdza czy pole czas jest ustawione
    fn ma_czas(&self) -> bool {
        self.czas.is_some()
    }

    // Ustawia czas dla daty
    fn ustaw_czas(&mut self, c: Czas) {
        self.czas = Some(c);
    }

    // Usuwa czas (ustawia None)
    fn usun_czas(&mut self) {
        self.czas = None;
    }
}

// Enum Priorytet z trzema poziomami: Niski, Sredni, Wysoki
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum Priorytet {
    Niski,
    Sredni,
    Wysoki,
}

// Struktura Zadanie reprezentuje jedno zadanie do wykonania
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
struct Zadanie {
    nazwa: String,         // Nazwa zadania
    opis: String,          // Opis zadania
    priorytet: Priorytet,  // Priorytet zadania
    termin: Data,          // Termin wykonania (data z czasem)
}

// Funkcja glowna - testuje wszystkie struktury i metody
fn main() {
    // Tworzy czas 9:30:00
    let czas1 = Czas::z_3(9, 30, 0);
    println!("czas1 = {}", czas1.na_napis());

    // Tworzy czas z lancucha znakow
    let czas2 = Czas::z_napisu("14-05-15", '-');
    println!("czas2 = {}", czas2.na_napis());

    // Tworzy date 1 stycznia 2021 bez czasu
    let mut data1 = Data::z_3(1, Miesiac::Styczen, 2021);
    println!("data1 = {}", data1.na_napis());

    // Ustawia czas 9:30:00 dla data1
    data1.ustaw_czas(czas1);
    println!("data1 z czasem = {}", data1.na_napis());
    println!("data1.ma_czas() = {}", data1.ma_czas());

    // Tworzy date 31 grudnia 2020 z tekstu
    let mut data2 = Data::z_napisu("31/12/2020", '/');
    println!("data2 = {}", data2.na_napis());

    // Usuwa czas z data2 (chociaz nie byl ustawiony)
    data2.usun_czas();
    println!("data2 po usunieciu czasu = {}", data2.na_napis());

    // Porownuje data1 i data2
    println!("data1 > data2 ? {}", data1 > data2);

    // Tworzy zadanie 1 z data1 i priorytetem srednim
    let zadanie1 = Zadanie {
        nazwa: "Kup chleb".into(),           // Tekst jako String
        opis: "Pelnoziarnisty".into(),      // Tekst jako String
        priorytet: Priorytet::Sredni,
        termin: data1.clone(),              // Klonuje data1
    };

    // Tworzy zadanie 2 z data2 i priorytetem wysokim
    let zadanie2 = Zadanie {
        nazwa: "Zakoncz raport".into(),
        opis: "Dane z grudnia".into(),
        priorytet: Priorytet::Wysoki,
        termin: data2.clone(),
    };

    // Tworzy wektor zadan
    let mut zadania = vec![zadanie1, zadanie2];

    // Sortuje zadania wg terminu i priorytetu
    zadania.sort();

    // Wypisuje posortowane zadania
    println!("Posortowane zadania:");
    for z in zadania {
        println!("{:?}", z); // Debug wypisuje cale zadanie
    }
}