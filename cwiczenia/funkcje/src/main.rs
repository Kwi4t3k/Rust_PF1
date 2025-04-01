use std::mem::transmute;

fn main() {
    hello_world();
    tell_height(160);
    human_id("Joel", 55, 182.0);

    let x = {
        let price = 5;
        let qty = 10;
        price * qty
    };
    println!("Result is: {}", x);

    println!("Wynik dodawania: {}", add(4, 5));

    //calling a bmi function
    let weight = 70.0;
    let height = 1.82;
    println!("BMI: {:.2}", bmi(height, weight));
}

// const _X = {
//     // code
// };

// Hoisting - can call function anywhere in your code
fn hello_world() {
    println!("Hello World!");
}

// you can insert values
fn tell_height(height: u32) {
    println!("My height is {} cm", height);
}

//you can insert more than one value
fn human_id(name: &str, age: u32, height: f32) {
    println!("My name is {}, I am {} years old, and my height is {} cm", name, age, height);
}

// functions returning values
fn add(a: i32, b: i32) -> i32 {
    a + b
}

// Expressions and Statements
// Expression: Anything that returns a value
// Statement: Anything that does not return a value

// Expression
// 5
// true & false
// add(3, 4)
// if condition {value} else {value2}
// ({code})
// ------------------------------------------
// Statement
// Almost all statements in Rust end with  ;

// Final example
// BMI = weight(kg) / height(m)^2

fn bmi (height: f32, weight: f32) -> f32 {
    weight / (height.powf(2.0))
}