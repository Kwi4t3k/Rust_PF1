fn main() {
    // Loop with brake

    // let mut a = 0;
    // loop {
    //     println!("Hello, world! {}", a);
    //     a += 1;
    //
    //     if a == 51 {
    //         break
    //     }
    // }


    // // Loop labels to disambiguate between multiple loops
    // 'outer: loop {
    //     println!("Entered the outer loop");
    //
    //     'inner: loop {
    //         println!("Entered the inner loop");
    //
    //         // This would break only the inner loop
    //         //break;
    //
    //         // This breaks the outer loop
    //         break 'outer;
    //     }
    //
    //     println!("This point will never be reached");
    // }
    //
    // println!("Exited the outer loop");


    // Looping through a collection with loop

    let a = [1, 2, 3, 4, 5, 6];
    let b = ["a", "b", "c", "d", "e", "f"] ;

    for element in a {
        print!("{element} ");
    }

    println!();

    for element in b {
        print!("{element} ");
    }
}
