fn swap(x: &mut i32, y: &mut i32) {
    let pom = *x;
    *x = *y;
    *y = pom;
}

fn sort(a: &mut i32, b: &mut i32, c: &mut i32) {
    if a > b {
        swap(a, b);
    }
    if b > c {
        swap(b, c);
    }
    if a > b {
        swap(a, b);
    }
}

fn main(){
    let mut a = 15;
    let mut b = 20;
    let mut c = 10;
    println!("stare: {a}, {b}, {c}");
    sort(&mut a, &mut b, &mut c);
    println!("nowe: {a}, {b}, {c}");
}