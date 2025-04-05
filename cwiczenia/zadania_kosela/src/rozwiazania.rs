### Programowanie strukturalne i napisy

1. Napisz funkcję `d2((x, y), (x, y)) -> f32`, która obliczy dystans
   pomiędzy punktami w przestrzeni *R^2*.

fn d2(p1: (f32, f32), p2: (f32, f32)) -> f32 {
    let x_1 = p1.0;
    let x_2 = p2.0;
    let y_1 = p1.1;
    let y_2 = p2.1;

    let d = ((x_2 - x_1).powf(2.0) + (y_2 - y_1).powf(2.0)).sqrt();

    d
}

fn main() {
    let p1 = (1.0, 2.0);
    let p2 = (4.0, 6.0);

    println!("{:?}", d2(p1, p2));
}

2. Napisz funkcję `d3((x, y, z), (x, y, z)) -> f32`, która obliczy dystans
   pomiędzy punktami w przestrzeni *R^3*.

fn d2(p1: (f32, f32, f32), p2: (f32, f32, f32)) -> f32 {
    let x_1 = p1.0;
    let x_2 = p2.0;
    let y_1 = p1.1;
    let y_2 = p2.1;
    let z_1 = p1.2;
    let z_2 = p2.2;

    let d = ((x_1 - x_2).powf(2.0) + (y_1 - y_2).powf(2.0) + (z_1 - z_2).powf(2.0)).sqrt();

    d
}

fn main() {
    let p1 = (0.0, 1.0, 0.0);
    let p2 = (0.0, 4.0, 0.0);

    println!("{:?}", d2(p1, p2));
}

3. Stwórz tablicę *N* elementów, którą wypełnisz resztami z dzielenia liczby
   `100` przez kolejne liczby naturalne. Następnie wyświetl kolejne wartości
   tablicy od końca.

fn main() {
    let mut vec: Vec<i32> = Vec::new();

    for i in 1..=100 {
        vec.push(100 % i);
    }

    for i in vec.iter().rev() {
        print!("{i} ");
    }
    println!();

    println!("{:?}", vec.iter().rev().collect::<Vec<_>>());


    // for i in 1..=100 {
    //     let reszta = 100 % i;
    //     println!("100 % {} = {}", i, reszta);
    // }
}

4. Napisz funkcję `avg(&[u32]) -> f32`, która obliczy średnią arytmetyczną
   liczb z tablicy.

fn avg(tab: &[u32]) -> f32 {
    let mut wynik = 0;
    let mut licznik = 0;

    for i in tab {
        wynik += i;
        licznik += 1;
    }

    wynik as f32 / licznik as f32
}

fn avg(tab: &[u32]) -> f32 {
    let suma: u32 = tab.iter().sum();
    let licznik = tab.len();

    suma as f32 / licznik as f32
}

fn main() {
    let tab: [u32; 5] = [1, 2, 3, 4, 3];
    println!("{}", avg(&tab));
}

5. Napisz funkcję `sort(... u32, ... u32, ... u32)`, która rosnąco posortuje
   przekazane jej argumenty.

fn swap(x: &mut u32, y: &mut u32) {
    let tmp = *x;
    *x = *y;
    *y = tmp;
}

fn sort(a: &mut u32, b: &mut u32, c: &mut u32) {
    if *a > *b {
        swap(a, b);
    }

    if *a > *c {
        swap(a, c);
    }

    if *b > *c {
        swap(b, c)
    }
}

fn main() {
    let mut a = 8;
    let mut b = 2;
    let mut c = 6;

    sort(&mut a, &mut b, &mut c);

    println!("{a} {b} {c}");
}

6. Napisz funkcję `swap_range(... [u32], (a1, a2), (b1, b2))`, która zamieni
   miejscami elementy, leżące w podanych przedziałach; jeśli przedziały mają
   różną długość, ogranicz się do długości krótszego z nich.

use std::cmp::min;

fn swap_range(tab: &mut [u32], a: (usize, usize), b: (usize, usize)) {
    let len = min(a.1 - a.0, b.1 - b.0);

    for i in 0..=len {
        let tmp = tab[a.0 + i];
        tab[a.0 + i] = tab[b.0 + i];
        tab[b.0 + i] = tmp;
    }
}

fn main() {
    let mut tab = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    swap_range(&mut tab, (0, 3), (8, 10));
    println!("{:?}", tab);
}

7. Napisz funkcję, która na podstawie napisu tworzy napis, zawierający co drugi znak napisu podanego w argumencie.

fn co_drugi_znak(napis: String) -> String {
    napis.chars().step_by(2).collect()
}

fn main() {
    let napis = String::from("123456");
    println!("{}", co_drugi_znak(napis));
}

### Option<> i Result<>

1. Napisz funkcję `fraction(numerator: i32, denominator: i32) -> Option<f32>`, która wykona odpowiednie dzielenie lub zwróci `None`, jeżeli to niemożliwe.
2. Napisz funkcję `position(element: i32, array: &[i32] -> Option<usize>)`. Funkcja powinna zwrócić indeks elementu w tablicy lub `None`, jeżeli element nie jest w tablicy.
3. Napisz funkcję `divisors(number: Option<u32>) -> usize`, która zwróci liczbę dzielników parametru number lub zakończy działanie programu, jeśli number ma wartość `None`.
4. Napisz funkcję `wizytowka(imie: Option<String>, nazwisko: Option<String>) -> String`, która stworzy wizytówkę, w której w przypadku braku imienia zostanie użyte imię Jan, a w przypadku braku nazwiska -- Kowalski.
5. Napisz funkcję `miejsce_zerowe(a: f32, b: f32, c: f32) -> (Option<f32>, Option<f32>)`, która oblicza rzeczywiste miejsca zerowe funkcji kwadratowej.
6. Napisz funkcję `oceny(punkty: &[u32], &mut[Result<u8, u32>])`, która przyporządkuje oceny studentom według pewnego klucza. Jeśli ktoś ma więcej niż 100 punktów, należy na tej pozycji umieścić wartość, informującą o błędzie i podać liczbę punktów ponad progiem.
7. Napisz funkcję `rozne_liczby(arr: &[&str], out: &mut [Result<u32, u32>])`, która przyjmie tablicę liczb zapisanych w postaci napisów w systemach dziesiętnym i szesnastkowym. Funkcja powinna rozpoznać system, w którym zapisana jest liczba i przekonwertować ją do zmiennej typu `u32`. Przyjmij, że liczby szesnastkowe oznaczone są prefiksem `0x`. Nie wszystkie napisy muszą być poprawne; zadbaj o obsługę błędów.

### Pętle i iteratory

1. Napisz funkcję o nagłówku `fn powtorki(t: ...) -> ...` która z danego
   wektora utworzy nowy z tymi samymi wartościami, ale tylko tymi, które się po
   sobie powtarzają. Na przykład:
   `powtorki(&vec![1, 3, 4, 3, 3, 3, 3, 4, 1, 1, 6]) == vec![3, 3, 3, 3, 1, 1]`

fn powtorki(t: &Vec<usize>) -> Vec<usize> {
   let mut result: Vec<usize> = Vec::new();

   for i in 0..t.len() - 1 {
      if t[i] == t[i+1] {
         result.push(t[i]);
      } else if i > 0 && t[i] == t[i-1] {
         result.push(t[i]);
      }
   }

   result
}

fn main() {
   print!("{:?}", powtorki(&vec![1, 3, 4, 3, 3, 3, 3, 4, 1, 1, 6]));
}

2. Napisz funkcję o nagłówku `fn unikalne(t: ...) -> ...` która utworzy i
   zwróci nowy wektor, ale tylko z wartościami, które w danym wektorze się nie
   powtarzają (w zachowanej kolejności). Na przykład:
`unikalne(&vec![1, 3, 4, 3, 3, 5, 3, 4, 1, 1, 6]) == vec![5, 6]`

fn unikalne(t: &Vec<usize>) -> Vec<usize> {
    let mut result = Vec::new(); // Tutaj będziemy przechowywać liczby unikalne

    for i in 0..t.len() {
        let mut counter = 0; // Licznik wystąpień danego elementu

        // Przechodzimy przez wszystkie elementy i zliczamy, ile razy dany element się powtarza
        for j in 0..t.len() {
            if t[i] == t[j] {
                counter += 1;
            }
        }

        // Jeśli licznik wskazuje dokładnie 1 wystąpienie, dodajemy element do wyniku
        if counter == 1 {
            result.push(t[i]);
        }
    }

    result // Zwracamy wektor z unikalnymi liczbami
}

fn main() {
    let dane = vec![1, 3, 4, 3, 3, 5, 3, 4, 1, 1, 6];
    println!("{:?}", unikalne(&dane)); // Oczekiwany wynik: [5, 6]
}

3. Napisz funkcję o nagłówku `fn zlicz_wiele(s1: ..., s2: ...) -> ...` która
   zwróci liczbę elementów (z powtórzeniami) wektora s1 zawartych w s2 (lub
           odwrotnie — wynik będzie ten sam).

```
zlicz_wiele(&vec![1, 2, 1, 3], &vec![1, 2]) == 3
zlicz_wiele(&vec![1, 2, 1, 3], &vec![12]) == 0
zlicz_wiele(&vec![1, 2, 1, 3], &vec![1, 2, 2]) == 4
zlicz_wiele(&vec![1, 2, 1, 3], &vec![1, 2, 2, 1]) == 6
```



### Napisy

Zadanie:
Napisz funkcję, która przyjmuje napis i zwraca nowy napis, w którym usunięte są wszystkie samogłoski. Funkcja powinna działać na małych i wielkich literach.

Przykład:
Wejście: "Hello World"

Wyjście: "Hll Wrld"

Funkcja:
rust
Kopiuj
fn usun_samogloski(napis: &str) -> String {
    // Twoje rozwiązanie
}

fn main() {
    let napis = "Hello World";
    println!("{}", usun_samogloski(napis));  // Oczekiwany wynik: "Hll Wrld"
}


fn usun_samogloski(napis: &str) -> String {
    napis.chars().filter(|x|
    match x {
        'a' | 'e' | 'i' | 'o' | 'u' => false,
        'A' | 'E' | 'I' | 'O' | 'U' => false,
        _ => true,
    }).collect()
}

fn main() {
    let napis = "Hello WorldOl";
    println!("{}", usun_samogloski(napis));  // Oczekiwany wynik: "Hll Wrld"
}


Zadanie:
Napisz funkcję, która zamieni wszystkie małe litery w napisie na wielkie, a wszystkie wielkie na małe. Funkcja powinna zwrócić nowy napis, w którym litery są zamienione w ten sposób, bez zmiany innych znaków (np. cyfr, spacji, znaków interpunkcyjnych itp.).

Przykład:
Wejście: "Hello World!"

Wyjście: "hELLO wORLD!"


fn zmien_litery(napis: &str) -> String {
    // Przechodzimy po każdym znaku w napisie
    napis
        .chars() // `chars()` zamienia napis na iterator znaków
        .map(|x|  // `map` pozwala na wykonanie operacji na każdym znaku
            match x {  // Sprawdzamy wartość znaku
                // Jeśli znak jest małą literą (a-z), zamieniamy go na dużą literę
                'a'..='z' => x.to_uppercase().next().unwrap(),  // `to_uppercase()` zamienia na dużą literę, a `next()` bierze pierwszy znak
                // Jeśli znak jest dużą literą (A-Z), zamieniamy go na małą literę
                'A'..='Z' => x.to_lowercase().next().unwrap(),  // `to_lowercase()` zamienia na małą literę, a `next()` bierze pierwszy znak
                // W przeciwnym wypadku zostawiamy znak bez zmian
                _ => x,  // Zwracamy znak bez zmian, jeśli nie jest to litera
            })
        .filter(|x|  // `filter` używamy do usunięcia niechcianych znaków
            match x {  // Sprawdzamy, które znaki mają pozostać
                // Jeśli znak jest samogłoską (małą lub dużą), zostawiamy go w napisie
                'a' | 'e' | 'i' | 'o' | 'u' => true,  // Małe samogłoski
                'A' | 'E' | 'I' | 'O' | 'U' => true,  // Duże samogłoski
                // Jeśli znak to wykrzyknik lub spacja, również go zostawiamy
                '!' | ' ' => true,  // Wykrzyknik i spacja
                // Wszystkie inne znaki zostają odrzucone
                _ => false,  // Inne znaki (konsonanty, cyfry, itp.) są odrzucane
            })
        .collect()  // Zbieramy wyniki z powrotem do ciągu znaków (String)
}

fn main() {
    let napis = "Hello World!";  // Przykładowy napis
    println!("{}", zmien_litery(napis));  // Oczekiwany wynik: "hEllO wOrLD!"
}


// Napisz funkcję count_vowels, która przyjmuje napis typu &str i zwraca liczbę samogłosk (małych liter) w tym napisie.

fn count_vowels(napis: &str) -> usize {
    napis.chars().filter(|x|
    match x {
        'a' | 'o' | 'u' | 'i' | 'e' => true,
        _ => false,
    }).count()
}

fn main() {
    let napis = "bananai"; // 4
    println!("{}", count_vowels(napis));
}

// Napisz funkcję reverse_words, która przyjmuje napis typu &str, odwraca kolejność słów w napisie i zwraca wynik jako nowy napis. Słowa są oddzielone spacjami.

fn reverse_napis(napis: &str) -> String {
    // Dzielimy napis na słowa, odwracamy ich kolejność, a następnie łączymy w jeden napis
    napis.split_whitespace() // Split na słowa, ignorując nadmiarowe spacje
        .rev() // Odwracamy kolejność słów
        .collect::<Vec<&str>>() // Zbieramy do wektora
        .join(" ") // Łączymy słowa w jeden napis, oddzielając spacjami
}

fn main() {
    let napis = "bananai ada";
    println!("{}", reverse_napis(napis)); // Oczekiwany wynik: "ada bananai"
}

// Napisz funkcję capitalize_first, która przyjmuje napis typu &str, a następnie zwraca nowy napis, w którym pierwsza litera jest wielka, a pozostałe litery małe. Pamiętaj, aby napisać to w sposób, który działa również w przypadku pustego napisu.

fn capitalize_first(napis: &str) -> String {
    if napis.is_empty() {
        napis.to_string()
    } else {
        let (pierwsza, reszta) = napis.split_at(1);
        pierwsza.to_uppercase() + &reszta.to_lowercase()
    }
}

fn capitalize_first(napis: &str) -> String {
    if napis.is_empty() {
        napis.to_string()
    } else {
        napisz[0..1].to_uppercase() + &napis[1..].to_lowercase()
    }
}

fn main() {
    // let napis = "bananai dsada dsad";
    let napis = "bananai";
    println!("{}", capitalize_first(napis));
}