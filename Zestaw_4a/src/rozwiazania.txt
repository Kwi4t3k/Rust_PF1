1. Napisz funkcję o nagłówku

fn liczba_wystapien(napis: ..., znak: ...) -> ...

która zliczy i zwróci ile jest danych znaków w danym napisie.

// Definicja funkcji liczba_wystapien, która przyjmuje napis (typ &str) oraz znak (typ char),
// a zwraca liczbę wystąpień tego znaku jako wartość typu i32
fn liczba_wystapien(napis: &str, znak: char) -> i32 {
    let mut count = 0; // Inicjalizujemy zmienną count do zliczania wystąpień znaku, początkowo 0

    let iterator = napis.chars(); // Tworzymy iterator, który pozwoli nam przejść przez każdy znak napisu

    for c in iterator { // Pętla for: iterujemy po każdym znaku napisu
        if c == znak { // Sprawdzamy, czy aktualny znak jest równy temu, którego szukamy
            count += 1; // Jeśli tak, zwiększamy licznik o 1
        }
    }

    count // Zwracamy wynik - łączną liczbę wystąpień znaku
}

fn main() {
    let napis = "napis adam"; // Definiujemy przykładowy napis
    let znak = 'a'; // Definiujemy znak, którego wystąpień chcemy szukać

    let wynik = liczba_wystapien(napis, znak); // Wywołujemy funkcję liczba_wystapien i zapisujemy wynik
    println!("Znak '{}' występuje {} razy w napisie.", znak, wynik); // Wypisujemy wynik na ekran
}

//--------------------------------------

// Definicja funkcji liczba_wystapien
// Przyjmuje napis (typ &str) i znak (typ char), zwraca liczbę wystąpień znaku jako usize
fn liczba_wystapien(napis: &str, znak: char) -> usize {
    // 1. Zamieniamy napis na iterator po znakach
    // 2. Filtrujemy te znaki, które są równe szukanemu znakowi
    // 3. Zliczamy pasujące znaki za pomocą .count()
    napis.chars().filter(|c| *c == znak).count()
    // napis.chars() – iterator po znakach napisu
    // filter(|c| *c == znak) – zostawia tylko te znaki, które są równe 'znak'
    // count() – zlicza liczbę takich znaków i zwraca jako wynik
}

fn main() {
    let napis = "napis"; // Definiujemy napis, w którym szukamy znaku
    let znak = 'a'; // Definiujemy znak, którego wystąpień chcemy szukać

    let wynik = liczba_wystapien(napis, znak); // Wywołujemy funkcję, wynik zapisujemy do zmiennej
    println!("Znak '{}' występuje {} razy w napisie.", znak, wynik); // Wypisujemy wynik na ekran
}

//--------------------------------------------------------------------------------------------------------------------------------------

2. Napisz funkcję o nagłówku
   fn rzymskie(napis: ...) -> ...
   która dla napisu reprezentującego liczbę w zapisie rzymskim (zakładamy jego poprawność) zwraca liczbę reprezentowaną przez ów napis. Przykłady:
   rzymskie("III") == 3
   rzymskie("IX") == 9
   rzymskie("XIX") == 19
   rzymskie("MCMX") == 1910


// Funkcja rzymskie przyjmuje napis (&str) w zapisie rzymskim i zwraca odpowiadającą mu wartość liczbową (u32)
fn rzymskie(napis: &str) -> u32 {
    let mut suma = 0; // Zmienna do przechowywania wyniku końcowego (sumy wartości znaków)
    let mut prev_value = 0; // Zmienna do przechowywania wartości poprzedniego znaku

    // Przechodzimy przez znaki w napisie, ale od końca (rev()), ponieważ łatwiej wtedy obsłużyć odejmowanie
    for c in napis.chars().rev() {
        // Dla każdego znaku ustalamy jego wartość liczbową
        let current_value = match c {
            'I' => 1,     // 'I' ma wartość 1
            'V' => 5,     // 'V' ma wartość 5
            'X' => 10,    // 'X' ma wartość 10
            'L' => 50,    // 'L' ma wartość 50
            'C' => 100,   // 'C' ma wartość 100
            'D' => 500,   // 'D' ma wartość 500
            'M' => 1000,  // 'M' ma wartość 1000
            _ => 0        // Jeśli znak jest niepoprawny (nie występuje w systemie rzymskim), przyjmujemy 0
        };

        // Jeżeli wartość obecnego znaku jest mniejsza niż poprzedniego, to odejmujemy (np. IV = 4, bo I < V)
        if current_value < prev_value {
            suma -= current_value;
        } else { // W przeciwnym wypadku dodajemy wartość do sumy
            suma += current_value;
        }

        prev_value = current_value; // Aktualizujemy wartość poprzedniego znaku
    }

    suma // Zwracamy obliczoną sumę
}

fn main() {
    // Przykłady testowe:
    println!("{}", rzymskie("III"));  // 3 (1+1+1)
    println!("{}", rzymskie("IX"));   // 9 (10-1)
    println!("{}", rzymskie("XIX"));  // 19 (10 + (10-1))
    println!("{}", rzymskie("MCMX")); // 1910 (1000 + (1000-100) + 10)
}