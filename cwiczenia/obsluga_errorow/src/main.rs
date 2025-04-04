fn divide_option(numerator: f64, denominator: f64) -> Option<f64> {
    if denominator == 0.0 {
        None
    } else {
        Some(numerator / denominator)
    }
}

fn divide_result(numerator: f64, denominator: f64) -> Result<f64, String> {
    if denominator == 0.0 {
        Err("Cannot divide by zero".to_string())
    } else {
        Ok(denominator / numerator)
    }
}

fn main() {
    // Approach 1
    enum Option<T>{ // Define the generic Option type
        Some(T), // Represents a value
        None, // Represents no value
    }

    let result_option = divide_option(10.0, 0.0);
    match result_option {
        Some(x) => println!("Result: {}", x),
        None => println!("Cannot divide by zero"),
    }

    // Approach 2
    enum Result<T, E>{ // Define the generic Result type
        Ok(T), // Represents a value
        Err(E), // Represents an error
    }

    match divide_result(100.23, 73.98) {
        Ok(result_result) => println!("Result: {}", result_result),
        Err(err) => println!("Error: {}", err),
    }
}
