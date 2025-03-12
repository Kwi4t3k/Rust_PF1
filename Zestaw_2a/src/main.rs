fn f(x: f64) -> f64 {
    x*x - 3.0*x + 2.0
}
fn fp(x: f64) -> f64 {
    2.0*x - 3.0
}

fn met_newt_for(x0: f64, eps: f64, n: u128) -> f64 {
    let mut x = x0;

    for _ in 0..n {
        if fp(x) < 0.0 {
            break;
        }

        let xp = x - f(x) / fp(x);

        if (xp - x).abs() < eps {
            return xp;
        }
        x = xp;
    }
    x
}

fn main() {
    let x0 = 4.0;
    let eps = 1.0;
    let n = 4;

    println!("{}", met_newt_for(x0, eps, n));
}