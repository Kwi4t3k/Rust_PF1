// Tuple
// "Alice" jest: string slice
// "Alice".to_string(): jest string

fn main() {
    let human: (String, i32, bool) = ("Alice".to_string(), 30, false);
    println!("Human Tuple: {:?}", human);

    let my_mix_tuple = ("Kratos", 23, true, [1, 2, 3, 4, 5]);
    println!("My Mix Tuple {:?}", my_mix_tuple);


    // Slices: [1, 2, 3, 4, 5]
    let number_slices: &[i32] = &[1, 2, 3, 4, 5];
    println!("Number Slice: {:?}", number_slices);

    let animal_slices: &[&str] = &["Lion", "Elephant", "Crocodile"];
    println!("Animal Slices: {:?}", animal_slices);

    let book_slices: &[&String] = &[&"IT".to_string(), &"Harry Potter".to_string(), &"ZEN".to_string()];
    println!("Book Slices: {:?}", book_slices);


    // String vs String Slices(&str) | String - kopiec(wolniejszy) , &str - stos(szybszy)
    // Strings [growable, mutable, owned string type]
    let mut stone_cold: String = String::from("Hell, ");
    println!("Stone cold says: {}", stone_cold);
    stone_cold.push_str("Yeah!");
    println!("Stone cold says: {}", stone_cold);

    // B- &str (String Slice)
    let string: String = String::from("Hello, World!");
    let slice: &str = &string[0..5];
    println!("slice: {}", slice);
}