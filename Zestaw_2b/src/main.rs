fn pow_mod(x: u128, n: u128, p: u128) -> u128 {
    if n == 0 {
        return 1;
    }
    if n % 2 == 0 {
        let a = pow_mod(x, n/2, p);
        (a * a) % p
    } else {
        (x % p * pow_mod(x, n-1, p)) % p
    }
}
fn main() {
    let x = 2;
    let n = 10;
    let p = 1000;

    println!("{}", pow_mod(x, n, p));


// x	n	p	    Oczekiwany wynik (xⁿ) % p
// 2	10	1000	    24
// 3	7	13	        3
// 5	3	13	        8
// 4	4	9	        4
// 10	5	17	        6
}