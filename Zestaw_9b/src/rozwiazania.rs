1. Stwórz strukturę Date oraz zaimplementuj dla niej następujące metody:
	- fn to_string(&self) -> String
	- fn from_3(day: u8, month: Month, year: u16) -> Date
	- fn from_string(string: &str, delim: char) -> Date

Zdefiniuj strukturę Date
struct Date {
    day: u8,
    month: Month,    // Month to Twój enum z nazwami miesięcy
    year: u16,
}

Zaimplementuj trzy niezależne metody:
impl Date {
    // a) zwraca "DD.MM.YYYY"
    fn to_string(&self) -> String { /* … */ }

    // b) konstruktor z trzech argumentów
    fn from_3(day: u8, month: Month, year: u16) -> Date { /* … */ }

    // c) parser z napisu z podanym separatorem
    fn from_string(s: &str, delim: char) -> Date { /* … */ }
}

2. Stwórz strukturę Time oraz zaimplementuj dla niej metody analogiczne jak dla struktury Date.

Zdefiniuj strukturę Time
struct Time {
    hour: u8,
    minute: u8,
    second: u8,
}

Załóż dla niej te same trzy metody:
impl Time {
    // a) "HH:MM:SS"
    fn to_string(&self) -> String { /* … */ }

    // b) from_3(hour, minute, second)
    fn from_3(hour: u8, minute: u8, second: u8) -> Time { /* … */ }

    // c) from_string("12-34-56", '-')
    fn from_string(s: &str, delim: char) -> Time { /* … */ }
}

3. Zmodyfikuj strukturę Date tak, aby mogła opcjonalnie przechowywać również czas. Dodaj metody:

	- fn has_time(&self) -> bool
	- fn set_time(&mut self, time: ...)
	- fn clear_time(&mut self)

struct Date {
    day: u8,
    month: Month,
    year: u16,
    time: Option<Time>,   // <— tu dodajemy
}

impl Date {
    // a) czy istnieje czas?
    fn has_time(&self) -> bool { /* … */ }

    // b) ustaw czas
    fn set_time(&mut self, t: Time) { /* … */ }

    // c) wyczyść czas
    fn clear_time(&mut self) { /* … */ }
}

4. W zgodzie ze zdroworozsądkowym poczuciem czasu zaimplementuj cechy: PartialOrd, Ord , PartialEq, Eq dla zmodyfikowanej struktury Date. Zastanów się nad pięknem ludzkiego postrzegania czasu oraz czasem-samym-w-sobie ;)

Porównywanie (PartialEq, Eq, PartialOrd, Ord) dla Date
Derywuj lub ręcznie zaimplementuj tak, aby kolejność była:

najpierw year,

potem month,

potem day,

a jeśli oba Date mają time = Some, to porównaj i Time (godzina → minuta → sekunda).

5. Stwórz strukturę Task. Powinna zawierać następujące pola:

	- name: String
	- description: String
	- priority: Priority // Low, Medium, High
	- due: Date

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum Priority {
    Low,
    Medium,
    High,
}

6. Zaimplementuj sensownie cechy PartialOrd, PartialEq dla struktury Task.

struct Task {
    name: String,
    description: String,
    priority: Priority,
    due: Date,
}

Derywuj lub zaimplementuj PartialEq/Eq oraz PartialOrd/Ord
– najpierw porównuj terminy (`due`),
– jeśli terminy równe, porównuj `priority`.
