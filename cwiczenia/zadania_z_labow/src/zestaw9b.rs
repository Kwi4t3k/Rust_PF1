// 1

#[derive(Debug, PartialEq, PartialOrd)]
enum Month {
    Styczen,
    Luty,
    Marzec,
    Kwiecien,
    Maj,
    Czerwiec,
    Lipiec,
    Sierpien,
    Wrzesien,
    Pazdziernik,
    Listopad,
    Grudzien,
}

#[derive(Debug, PartialEq)]
struct Date {
    day: u8,
    month: Month,
    year: u16,
}

impl Date {
    fn to_string(&self) -> String {
        format!("{:02}-{:?}-{}", self.day, self.month, self.year)
    }

    fn from_3(day: u8, month: Month, year: u16) -> Self {
        Self {
            day,
            month,
            year,
        }
    }

    fn from_string(string: &str, delim: char) -> Self {
        let parts: Vec<&str> = string.split(delim).collect();

        let day = parts[0].parse().unwrap();

        let month: Month = match parts[1].to_lowercase().as_str() {
            "styczen" => Month::Styczen,
            "luty" => Month::Luty,
            "marzec" => Month::Marzec,
            "kwiecien" => Month::Kwiecien,
            "maj" => Month::Maj,
            "czerwiec" => Month::Czerwiec,
            "lipiec" => Month::Lipiec,
            "sierpien" => Month::Sierpien,
            "wrzesien" => Month::Wrzesien,
            "pazdziernik" => Month::Pazdziernik,
            "listopad" => Month::Listopad,
            "grudzien" => Month::Grudzien,
            _ => panic!("Nieprawidłowy miesiąc")
        };

        let year: u16 = parts[2].parse().unwrap();

        Self {
            day,
            month,
            year,
        }
    }
}

fn main() {
    // 1) Konstruktor from_3
    let d1 = Date::from_3(5, Month::Marzec, 2025);
    println!("Data 1: {}", d1.to_string());
    // -> "5 Marzec 2025"

    // 2) Parsowanie z napisu – tutaj miesiące po polsku, rozdzielone myślnikiem
    let d2 = Date::from_string("12-Kwiecien-1998", '-');
    println!("Data 2: {}", d2.to_string());
    // -> "12 Kwiecien 1998"

    // 3) Inny separator i inny rok
    let d3 = Date::from_string("31/Grudzien/1999", '/');
    println!("Data 3: {}", d3.to_string());
    // -> "31 Grudzien 1999"

    // 4) Kilka asercji, żeby upewnić się, że działa poprawnie
    assert_eq!(d1.day, 5);
    assert_eq!(d1.month, Month::Marzec);
    assert_eq!(d1.year, 2025);

    assert_eq!(d2, Date { day: 12, month: Month::Kwiecien, year: 1998 });
    assert_eq!(d3, Date::from_3(31, Month::Grudzien, 1999));

}

// 2

#[derive(Debug, PartialEq, PartialOrd)]
enum Month {
    Styczen,
    Luty,
    Marzec,
    Kwiecien,
    Maj,
    Czerwiec,
    Lipiec,
    Sierpien,
    Wrzesien,
    Pazdziernik,
    Listopad,
    Grudzien,
}

struct Date {
    day: u8,
    month: Month,
    year: u16,
}

impl Date {
    fn to_string(&self) -> String {
        format!("{:02}-{:?}-{}", self.day, self.month, self.year)
    }

    fn from_3(day: u8, month: Month, year: u16) -> Self {
        Self {
            day,
            month,
            year,
        }
    }

    fn from_string(string: &str, delim: char) -> Self {
        let parts: Vec<&str> = string.split(delim).collect();

        let day = parts[0].parse().unwrap();

        let month: Month = match parts[1].to_lowercase().as_str() {
            "styczen" => Month::Styczen,
            "luty" => Month::Luty,
            "marzec" => Month::Marzec,
            "kwiecien" => Month::Kwiecien,
            "maj" => Month::Maj,
            "czerwiec" => Month::Czerwiec,
            "lipiec" => Month::Lipiec,
            "sierpien" => Month::Sierpien,
            "wrzesien" => Month::Wrzesien,
            "pazdziernik" => Month::Pazdziernik,
            "listopad" => Month::Listopad,
            "grudzien" => Month::Grudzien,
            _ => panic!("Nieprawidłowy miesiąc")
        };

        let year: u16 = parts[2].parse().unwrap();

        Self {
            day,
            month,
            year,
        }
    }
}

struct Time {
    hour: u8,
    minute: u8,
    second: u8,
}

impl Time {
    fn to_string(&self) -> String {
        format!("{:02}-{:02}-{:02}", self.hour, self.minute, self.second)
    }

    fn from_3(hour: u8, minute: u8, second: u8) -> Self {
        Self {
            hour,
            minute,
            second,
        }
    }

    fn from_string(string: &str, delim: char) -> Self {
        let parts: Vec<&str> = string.split(delim).collect();

        let hour = parts[0].parse().unwrap();
        let minute = parts[1].parse().unwrap();
        let second = parts[2].parse().unwrap();

        Self {
            hour,
            minute,
            second,
        }
    }
}

// 3

#[derive(Debug, PartialEq, PartialOrd)]
enum Month {
    Styczen,
    Luty,
    Marzec,
    Kwiecien,
    Maj,
    Czerwiec,
    Lipiec,
    Sierpien,
    Wrzesien,
    Pazdziernik,
    Listopad,
    Grudzien,
}

struct Date {
    day: u8,
    month: Month,
    year: u16,
    time: Option<Time>
}

impl Date {
    fn to_string(&self) -> String {
        let date = format!("{:02}-{:?}-{}", self.day, self.month, self.year);

        if self.has_time() {
            return format!("{} {}", date, self.time.as_ref().unwrap().to_string());
        }

        return date;
    }

    fn from_3(day: u8, month: Month, year: u16) -> Self {
        Self {
            day,
            month,
            year,
            time: None,
        }
    }

    fn from_string(string: &str, delim: char) -> Self {
        let parts: Vec<&str> = string.split(delim).collect();

        let day = parts[0].parse().unwrap();

        let month: Month = match parts[1].to_lowercase().as_str() {
            "styczen" => Month::Styczen,
            "luty" => Month::Luty,
            "marzec" => Month::Marzec,
            "kwiecien" => Month::Kwiecien,
            "maj" => Month::Maj,
            "czerwiec" => Month::Czerwiec,
            "lipiec" => Month::Lipiec,
            "sierpien" => Month::Sierpien,
            "wrzesien" => Month::Wrzesien,
            "pazdziernik" => Month::Pazdziernik,
            "listopad" => Month::Listopad,
            "grudzien" => Month::Grudzien,
            _ => panic!("Nieprawidłowy miesiąc")
        };

        let year: u16 = parts[2].parse().unwrap();

        Self {
            day,
            month,
            year,
            time: None,
        }
    }

    fn has_time(&self) -> bool {
        match self.time {
            None => false,
            Some(_) => true,
        }
    }

    fn set_time(&mut self, time: Time) {
        self.time = Some(time)
    }

    fn clear_time(&mut self) {
        self.time = None
    }
}

struct Time {
    hour: u8,
    minute: u8,
    second: u8,
}

impl Time {
    fn to_string(&self) -> String {
        format!("{:02}-{:02}-{:02}", self.hour, self.minute, self.second)
    }

    fn from_3(hour: u8, minute: u8, second: u8) -> Self {
        Self {
            hour,
            minute,
            second,
        }
    }

    fn from_string(string: &str, delim: char) -> Self {
        let parts: Vec<&str> = string.split(delim).collect();

        let hour = parts[0].parse().unwrap();
        let minute = parts[1].parse().unwrap();
        let second = parts[2].parse().unwrap();

        Self {
            hour,
            minute,
            second,
        }
    }
}

// 4

#[derive(Debug, PartialEq, PartialOrd)]
enum Month {
    Styczen,
    Luty,
    Marzec,
    Kwiecien,
    Maj,
    Czerwiec,
    Lipiec,
    Sierpien,
    Wrzesien,
    Pazdziernik,
    Listopad,
    Grudzien,
}

struct Date {
    day: u8,
    month: Month,
    year: u16,
    time: Option<Time>
}

impl Date {
    fn to_string(&self) -> String {
        let date = format!("{:02}-{:?}-{}", self.day, self.month, self.year);

        if self.has_time() {
            return format!("{} {}", date, self.time.as_ref().unwrap().to_string());
        }

        return date;
    }

    fn from_3(day: u8, month: Month, year: u16) -> Self {
        Self {
            day,
            month,
            year,
            time: None,
        }
    }

    fn from_string(string: &str, delim: char) -> Self {
        let parts: Vec<&str> = string.split(delim).collect();

        let day = parts[0].parse().unwrap();

        let month: Month = match parts[1].to_lowercase().as_str() {
            "styczen" => Month::Styczen,
            "luty" => Month::Luty,
            "marzec" => Month::Marzec,
            "kwiecien" => Month::Kwiecien,
            "maj" => Month::Maj,
            "czerwiec" => Month::Czerwiec,
            "lipiec" => Month::Lipiec,
            "sierpien" => Month::Sierpien,
            "wrzesien" => Month::Wrzesien,
            "pazdziernik" => Month::Pazdziernik,
            "listopad" => Month::Listopad,
            "grudzien" => Month::Grudzien,
            _ => panic!("Nieprawidłowy miesiąc")
        };

        let year: u16 = parts[2].parse().unwrap();

        Self {
            day,
            month,
            year,
            time: None,
        }
    }

    fn has_time(&self) -> bool {
        match self.time {
            None => false,
            Some(_) => true,
        }
    }

    fn set_time(&mut self, time: Time) {
        self.time = Some(time)
    }

    fn clear_time(&mut self) {
        self.time = None
    }
}

struct Time {
    hour: u8,
    minute: u8,
    second: u8,
}

impl Time {
    fn to_string(&self) -> String {
        format!("{:02}-{:02}-{:02}", self.hour, self.minute, self.second)
    }

    fn from_3(hour: u8, minute: u8, second: u8) -> Self {
        Self {
            hour,
            minute,
            second,
        }
    }

    fn from_string(string: &str, delim: char) -> Self {
        let parts: Vec<&str> = string.split(delim).collect();

        let hour = parts[0].parse().unwrap();
        let minute = parts[1].parse().unwrap();
        let second = parts[2].parse().unwrap();

        Self {
            hour,
            minute,
            second,
        }
    }
}

impl PartialEq for Date {
    fn eq(&self, other: &Self) -> bool {
        self.day == other.day &&
            self.month == other.month &&
            self.year == other.year &&
            self.time == other.time
    }
}

impl PartialEq for Time {
    fn eq(&self, other: &Self) -> bool {
        self.hour == other.hour && self.minute == other.minute && self.second==other.second
    }
}

impl PartialOrd for Date {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match self.year.partial_cmp(&other.year) {
            Some(std::cmp::Ordering::Equal) => {},
            ord => return ord,
        }
        match self.month.partial_cmp(&other.month) {
            Some(std::cmp::Ordering::Equal) => {},
            ord => return ord,
        }
        match self.day.partial_cmp(&other.day) {
            Some(std::cmp::Ordering::Equal) => {},
            ord => return ord,
        }

        self.time.partial_cmp(&other.time)
    }
}

impl Eq for Time {}
impl Eq for Date {}
impl Ord for Time {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.partial_cmp(other).unwrap()
    }
}

impl Ord for Date {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.partial_cmp(other).unwrap()
    }
}

impl PartialOrd for Time {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match self.hour.partial_cmp(&other.hour) {
            Some(std::cmp::Ordering::Equal) => {},
            ord => return ord,
        }
        match self.minute.partial_cmp(&other.minute) {
            Some(std::cmp::Ordering::Equal) => {},
            ord => return ord,
        }

        self.second.partial_cmp(&other.second)
    }
}

// 5

#[derive(Debug, PartialEq, PartialOrd)]
enum Month {
    Styczen,
    Luty,
    Marzec,
    Kwiecien,
    Maj,
    Czerwiec,
    Lipiec,
    Sierpien,
    Wrzesien,
    Pazdziernik,
    Listopad,
    Grudzien,
}

struct Date {
    day: u8,
    month: Month,
    year: u16,
    time: Option<Time>
}

impl Date {
    fn to_string(&self) -> String {
        let date = format!("{:02}-{:?}-{}", self.day, self.month, self.year);

        if self.has_time() {
            return format!("{} {}", date, self.time.as_ref().unwrap().to_string());
        }

        return date;
    }

    fn from_3(day: u8, month: Month, year: u16) -> Self {
        Self {
            day,
            month,
            year,
            time: None,
        }
    }

    fn from_string(string: &str, delim: char) -> Self {
        let parts: Vec<&str> = string.split(delim).collect();

        let day = parts[0].parse().unwrap();

        let month: Month = match parts[1].to_lowercase().as_str() {
            "styczen" => Month::Styczen,
            "luty" => Month::Luty,
            "marzec" => Month::Marzec,
            "kwiecien" => Month::Kwiecien,
            "maj" => Month::Maj,
            "czerwiec" => Month::Czerwiec,
            "lipiec" => Month::Lipiec,
            "sierpien" => Month::Sierpien,
            "wrzesien" => Month::Wrzesien,
            "pazdziernik" => Month::Pazdziernik,
            "listopad" => Month::Listopad,
            "grudzien" => Month::Grudzien,
            _ => panic!("Nieprawidłowy miesiąc")
        };

        let year: u16 = parts[2].parse().unwrap();

        Self {
            day,
            month,
            year,
            time: None,
        }
    }

    fn has_time(&self) -> bool {
        match self.time {
            None => false,
            Some(_) => true,
        }
    }

    fn set_time(&mut self, time: Time) {
        self.time = Some(time)
    }

    fn clear_time(&mut self) {
        self.time = None
    }
}

struct Time {
    hour: u8,
    minute: u8,
    second: u8,
}

impl Time {
    fn to_string(&self) -> String {
        format!("{:02}-{:02}-{:02}", self.hour, self.minute, self.second)
    }

    fn from_3(hour: u8, minute: u8, second: u8) -> Self {
        Self {
            hour,
            minute,
            second,
        }
    }

    fn from_string(string: &str, delim: char) -> Self {
        let parts: Vec<&str> = string.split(delim).collect();

        let hour = parts[0].parse().unwrap();
        let minute = parts[1].parse().unwrap();
        let second = parts[2].parse().unwrap();

        Self {
            hour,
            minute,
            second,
        }
    }
}

impl PartialEq for Date {
    fn eq(&self, other: &Self) -> bool {
        self.day == other.day &&
            self.month == other.month &&
            self.year == other.year &&
            self.time == other.time
    }
}

impl PartialEq for Time {
    fn eq(&self, other: &Self) -> bool {
        self.hour == other.hour && self.minute == other.minute && self.second==other.second
    }
}

impl PartialOrd for Date {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match self.year.partial_cmp(&other.year) {
            Some(std::cmp::Ordering::Equal) => {},
            ord => return ord,
        }
        match self.month.partial_cmp(&other.month) {
            Some(std::cmp::Ordering::Equal) => {},
            ord => return ord,
        }
        match self.day.partial_cmp(&other.day) {
            Some(std::cmp::Ordering::Equal) => {},
            ord => return ord,
        }

        self.time.partial_cmp(&other.time)
    }
}

impl Eq for Time {}
impl Eq for Date {}
impl Ord for Time {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.partial_cmp(other).unwrap()
    }
}

impl Ord for Date {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.partial_cmp(other).unwrap()
    }
}

impl PartialOrd for Time {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match self.hour.partial_cmp(&other.hour) {
            Some(std::cmp::Ordering::Equal) => {},
            ord => return ord,
        }
        match self.minute.partial_cmp(&other.minute) {
            Some(std::cmp::Ordering::Equal) => {},
            ord => return ord,
        }

        self.second.partial_cmp(&other.second)
    }
}

enum Priority {
    Low,
    Medium,
    High,
}

struct Task {
    name: String,
    description: String,
    priority: Priority,
    due: Date,
}

impl Task {
    fn from_4(name: &str, description: &str, priority: Priority, due: Date) -> Self {
        Self {
            name: name.to_string(),
            description: description.to_string(),
            priority,
            due,
        }
    }
}

// 6

// Definicja wyliczenia Month z możliwością debugowania i porównywania
#[derive(Debug, PartialEq, PartialOrd)]
enum Month {
    Styczen,    // 0: styczeń
    Luty,       // 1: luty
    Marzec,     // 2: marzec
    Kwiecien,   // 3: kwiecień
    Maj,        // 4: maj
    Czerwiec,   // 5: czerwiec
    Lipiec,     // 6: lipiec
    Sierpien,   // 7: sierpień
    Wrzesien,   // 8: wrzesień
    Pazdziernik,// 9: październik
    Listopad,   // 10: listopad
    Grudzien,   // 11: grudzień
}

// Struktura Date przechowująca dzień, miesiąc, rok oraz opcjonalny czas
struct Date {
    day: u8,             // numer dnia [1..31]
    month: Month,        // miesiąc jako enum Month
    year: u16,           // rok, np. 2025
    time: Option<Time>,  // opcjonalny czas dania (None = brak czasu)
}

impl Date {
    /// Tworzy napis w formacie "DD-Month-YYYY [HH:MM:SS]" lub bez czasu
    fn to_string(&self) -> String {
        // najpierw tworzymy część daty "DD-Month-YYYY"
        let date = format!("{:02}-{:?}-{}", self.day, self.month, self.year);
        // jeżeli mamy ustawiony czas, doklejamy spację i czas
        if self.has_time() {
            return format!(
                "{} {}",
                date,
                self.time.as_ref().unwrap().to_string() // unwrap bezpieczny, bo has_time()==true
            );
        }
        // w przeciwnym razie zwracamy tylko datę
        date
    }

    /// Zwraca true jeśli opcjonalny czas jest Some(_)
    fn has_time(&self) -> bool {
        match self.time {
            None    => false,  // brak czasu
            Some(_) => true,   // czas jest ustawiony
        }
    }

    /// Ustawia czas w obiekcie Date (zastępuje poprzedni)
    fn set_time(&mut self, time: Time) {
        self.time = Some(time);
    }

    /// Usuwa czas, zostawiając tylko datę
    fn clear_time(&mut self) {
        self.time = None;
    }

    /// Konstruktor z 3 argumentów day, month, year – time = None
    fn from_3(day: u8, month: Month, year: u16) -> Self {
        Self { day, month, year, time: None }
    }

    /// Parsuje z napisu "DD<delim>MM<delim>YYYY", miesiąc po polsku bez diakrytyków
    fn from_string(string: &str, delim: char) -> Self {
        // dzielenie napisu zgodnie z delim
        let parts: Vec<&str> = string.split(delim).collect();
        // parsowanie dnia, unwrap dopuszcza panic w razie błędu
        let day = parts[0].parse().unwrap();
        // dopasowanie napisu do wariantu Month
        let month: Month = match parts[1].to_lowercase().as_str() {
            "styczen"    => Month::Styczen,
            "luty"       => Month::Luty,
            "marzec"     => Month::Marzec,
            "kwiecien"   => Month::Kwiecien,
            "maj"        => Month::Maj,
            "czerwiec"   => Month::Czerwiec,
            "lipiec"     => Month::Lipiec,
            "sierpien"   => Month::Sierpien,
            "wrzesien"   => Month::Wrzesien,
            "pazdziernik"=> Month::Pazdziernik,
            "listopad"   => Month::Listopad,
            "grudzien"   => Month::Grudzien,
            _            => panic!("Nieprawidłowy miesiąc")
        };
        // parsowanie roku
        let year: u16 = parts[2].parse().unwrap();
        // budujemy instancję
        Self { day, month, year, time: None }
    }
}

// Struktura Time przechowująca godzinę, minutę i sekundę
struct Time {
    hour: u8,    // godzina [0..23]
    minute: u8,  // minuta [0..59]
    second: u8,  // sekunda [0..59]
}

impl Time {
    /// Formatuje czas jako "HH:MM:SS" z zerami wiodącymi
    fn to_string(&self) -> String {
        format!("{:02}:{:02}:{:02}", self.hour, self.minute, self.second)
    }

    /// Konstruktor z 3 liczb hour, minute, second
    fn from_3(hour: u8, minute: u8, second: u8) -> Self {
        Self { hour, minute, second }
    }

    /// Parsuje z napisu "HH:MM:SS", zakładamy poprawny format
    fn from_string(string: &str) -> Self {
        let parts: Vec<&str> = string.split(':').collect();
        let hour   = parts[0].parse().unwrap();
        let minute = parts[1].parse().unwrap();
        let second = parts[2].parse().unwrap();
        Self { hour, minute, second }
    }
}

// Ręczna implementacja porównywania Date (rok > miesiąc > dzień > czas)
impl PartialEq for Date {
    fn eq(&self, other: &Self) -> bool {
        self.day   == other.day &&
        self.month == other.month &&
        self.year  == other.year &&
        self.time  == other.time
    }
}
impl Eq for Date {} // Date spełnia Eq bo PartialEq jest totalne

impl PartialOrd for Date {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        // porównaj najpierw rok
        match self.year.partial_cmp(&other.year) {
            Some(std::cmp::Ordering::Equal) => {},
            ord => return ord,
        }
        // jeżeli rok równy, porównaj miesiąc
        match self.month.partial_cmp(&other.month) {
            Some(std::cmp::Ordering::Equal) => {},
            ord => return ord,
        }
        // jeżeli miesiąc równy, porównaj dzień
        match self.day.partial_cmp(&other.day) {
            Some(std::cmp::Ordering::Equal) => {},
            ord => return ord,
        }
        // na końcu porównaj Option<Time> (None < Some, a potem wewnętrzny cmp)
        self.time.partial_cmp(&other.time)
    }
}
impl Ord for Date {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.partial_cmp(other).unwrap()
    }
}

// Ręczna implementacja porównywania Time (hour > minute > second)
impl PartialEq for Time {
    fn eq(&self, other: &Self) -> bool {
        self.hour   == other.hour &&
        self.minute == other.minute &&
        self.second == other.second
    }
}
impl Eq for Time {} // Time to Eq
impl PartialOrd for Time {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match self.hour.partial_cmp(&other.hour) {
            Some(std::cmp::Ordering::Equal) => {},
            ord => return ord,
        }
        match self.minute.partial_cmp(&other.minute) {
            Some(std::cmp::Ordering::Equal) => {},
            ord => return ord,
        }
        self.second.partial_cmp(&other.second)
    }
}
impl Ord for Time {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.partial_cmp(other).unwrap()
    }
}

// Priorytety zadań: Low < Medium < High
#[derive(PartialEq, PartialOrd)]
enum Priority {
    Low,    // niski priorytet
    Medium, // średni priorytet
    High,   // wysoki priorytet
}

// Struktura Task łącząca nazwę, opis, priorytet i termin (Date)
#[derive(PartialEq)]
struct Task {
    name: String,      // nazwa zadania
    description: String,// opis zadania
    priority: Priority,// priorytet
    due: Date,         // termin wykonania
}

impl Task {
    /// Konstruktor z 4 argumentów: name, description, priority, due
    fn from_4(name: &str, description: &str, priority: Priority, due: Date) -> Self {
        Self {
            name: name.to_string(),
            description: description.to_string(),
            priority,
            due,
        }
    }
}

// Porównanie dwóch zadań: najpierw priorytet, potem termin
impl PartialOrd for Task {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        // jeśli priorytety różne, zwróć wynik porównania priorytetów
        match self.priority.partial_cmp(&other.priority) {
            Some(std::cmp::Ordering::Equal) => {},
            ord => return ord,
        }
        // w przeciwnym razie porównaj terminy
        self.due.partial_cmp(&other.due)
    }
}

fn main() {
    // Tworzymy trzy daty: d1 i d3 z parsingiem, d2 ręcznie
    let mut date1 = Date::from_string("13-Maj-2025", '-');
    let mut date2 = Date::from_3(13, Month::Maj, 2025);
    let mut date3 = Date::from_string("13-Maj-2025", '-');

    // Sprawdzenie, że month==Maj działa
    println!("{}", date1.month == Month::Maj);
    println!("{}", date1.month == date2.month);

    // Wyświetlenie samej daty (bez czasu)
    println!("{}", date1.to_string());
    println!("{}", date2.to_string());

    // Ustawienie różnych czasów
    date1.set_time(Time::from_3(11, 0, 0));        // 11:00:00
    date2.set_time(Time::from_string("10:59:59")); // 10:59:59
    date3.set_time(Time::from_string("11:00:01")); // 11:00:01

    // Wyświetlenie dat z czasem
    println!("{}", date1.to_string());
    println!("{}", date2.to_string());
    println!("{}", date3.to_string());

    // Porównania dat z czasu (date1 > date2? date1 < date3?)
    println!("{}", date1 > date2);
    println!("{}", date1 < date3);

    // Usuwamy czas z date1 i ponownie porównujemy
    date1.clear_time();
    println!("{}", date1.to_string());      // tylko data
    println!("{}", date1 < date2);         // None < Some(_)
    println!("{}", date1 < date3);         // None < Some(_)

    // Usuwamy czas z date3, teraz date1==date3 (oba bez czasu)
    date3.clear_time();
    println!("{}", date1 == date3);

    // Tworzymy dwa zadania z tymi samymi priorytetami, różnymi terminami
    let task1 = Task::from_4("t1", "opis1", Priority::High, date1);
    let task2 = Task::from_4("task2", "opis inny", Priority::High, date3);

    // Sprawdzamy operatory >, <, ==, != dla Task
    println!("{}", (task1 > task2) == false);
    println!("{}", (task1 < task2) == false);
    println!("{}", (task1 == task2) == false);
    println!("{}", (task1 != task2));
}