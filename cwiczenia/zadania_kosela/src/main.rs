// 3. Stwórz tablicę *N* elementów, którą wypełnisz resztami z dzielenia liczby
// `100` przez kolejne liczby naturalne. Następnie wyświetl kolejne wartości
// tablicy od końca.

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