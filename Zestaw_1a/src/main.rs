fn main() {
    let dana = 30;

    for c in 1..=dana {
        for b in 1..c {
            for a in 1..b {
                if a*a + b*b == c*c {
                    println!("({a}, {b}, {c})");
                }
            }
        }
    }
}