fn swap_2(a: &mut i32, b: &mut i32) {
    let tmp = *a;
    *a = *b;
    *b = tmp;
}

fn swap_3(a: &mut i32, b: &mut i32, c: &mut i32) {
    if a > b {
        swap_2(a, b);
    }
    if b > c {
        swap_2(b, c);
    }
    if a > b {
        swap_2(a, b);
    }
}

fn rand(seed: &mut usize, min_rand: usize, max_rand: usize) -> usize {
    *seed = (*seed * 134775813 + 1) % 4294967296;
    *seed % (max_rand - min_rand + 1) + min_rand
}

fn swap_arr(arr: &mut Vec<usize>, i: usize, j: usize) {
    let temp = arr[i];
    arr[i] = arr[j];
    arr[j] = temp;
}

fn rand_perm(arr: &mut Vec<usize>, seed: &mut usize) {
    let n = arr.len();

    for i in (1..n).rev() {
        let j = rand(seed, 0, i);
        swap_arr(arr, i, j);
    }
}

fn main() {
    let mut a = 5;
    let mut b = 10;

    println!("{a} {b}");
    swap_2(&mut a, &mut b);
    println!("{a} {b}");

    println!("-------------------");

    let mut a = 5;
    let mut b = 10;
    let mut c = 1;

    println!("{a} {b} {c}");
    swap_3(&mut a, &mut b, &mut c);
    println!("{a} {b} {c}");

    println!("-------------------");

    let mut a = 10;
    let b = 15;
    let c = 30;

    println!("{}", rand(&mut a, b, c));
    println!("{}", rand(&mut a, b, c));
    println!("{}", rand(&mut a, b, c));

    println!("-------------------");

    let mut vec = vec![1, 2, 3, 4, 5];
    println!("{:?}", vec);

    swap_arr(&mut vec, 0, 2);

    println!("{:?}", vec);

    println!("-------------------");

    let mut vec = vec![1, 2, 3, 4, 5];
    let mut seed = 10;

    println!("{:?}", vec);

    rand_perm(&mut vec, &mut seed);

    println!("{:?}", vec);
}