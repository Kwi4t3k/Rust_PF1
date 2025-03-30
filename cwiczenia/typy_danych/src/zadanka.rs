// zrób tablicę liczb, odfiltruj nieparzyste

// fn main() {
//     let tab = vec![1, 2, 3, 4, 5];
//     let parzyste: Vec<_> = tab.iter().filter(|x| *x % 2 == 0).collect();
//     println!("{:?}", parzyste);
// }

fn main() {
    let tab = [1, 2, 3, 4, 5];
    for i in tab.iter() {
        if i % 2 == 0 {
            print!("{i} ");
        }
    }
}

// 🔹 Zadanie:
// Napisz program w języku Rust, który:
//
// Tworzy tablicę liczb całkowitych.
//
// Wypisuje tylko te liczby, które są większe od 10.

fn main() {
    let tab = [1, 20, 3, 40, 5, 60];
    for i in tab.iter() {
        if *i > 10 {
           print!("{i} ")
        }
    }
}