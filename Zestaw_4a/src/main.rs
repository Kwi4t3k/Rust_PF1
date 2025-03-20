fn rzymskie(napis: &str) -> u32 {
    let mut suma = 0;
    let mut prev_value = 0;

    for c in napis.chars() {
        let current_value = match c {
            'I' => 1,
            'V' => 5,
            'X' => 10,
            'L' => 50,
            'C' => 100,
            'D' => 500,
            'M' => 1000,
            _ => 0
        };

        if current_value > prev_value {
            suma += current_value - 2 * prev_value;
        } else {
            suma += current_value;
        }

        prev_value = current_value;
    }

    suma
}

fn main() {
    println!("{}", rzymskie("III")); // 3
    println!("{}", rzymskie("IX"));  // 9
    println!("{}", rzymskie("XIX")); // 19
    println!("{}", rzymskie("MCMX"));// 1910
}