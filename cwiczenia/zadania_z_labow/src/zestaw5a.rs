// 1

fn zamien_syst8_na_syst2(z: &str) -> Option<String> {
    if z.is_empty() {
        return None;
    }

    for c in z.chars() {
        if !c.is_digit(8) {
            return None;
        }
    }

    let liczba = i32::from_str_radix(z, 8).unwrap();
    Some(format!("{:b}", liczba))
}

fn main() {
    println!("{:?}", zamien_syst8_na_syst2("6"));
}

fn zamien_syst8_na_syst2(z: &str) -> Option<String> {
    if z.is_empty() {
        return None;
    }
    let values = ["000", "001", "010", "011", "100", "101", "110", "111"];
    let mut res = String::new();
    for c in z.chars() {
        let value = c.to_digit(8)? as usize;
        let value = values[value];
        res += value;
    }

    Some(res.trim_start_matches('0').to_string())
}

fn main() {
    println!("{:?}", zamien_syst8_na_syst2("6"));
}

// 2

// 3