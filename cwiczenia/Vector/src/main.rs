fn main() {
    let v: Vec<i32> = Vec::new();

    let the_vec = vec![1, 2, 3, 4];

    let mut the_numbers_vec = Vec::new();
    the_numbers_vec.push(5);
    the_numbers_vec.push(6);
    the_numbers_vec.push(7);
    the_numbers_vec.push(8);

    println!("{:?}", the_numbers_vec);
    println!("{:?}", the_numbers_vec[1]);

    let third: &i32 = &the_numbers_vec[2];
    println!("{third}");

    let third = the_numbers_vec.get(2);
    match third {
        Some(third) => println!("The third element is: {}", third),
        None => println!("There is no third element"),
    }
}