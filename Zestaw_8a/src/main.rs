// zad 1

struct RandGen {
    seed: i64
}

impl RandGen {
    fn new(seed: i64) -> Self {
        Self { seed }
    }

    fn gen_range(&mut self, min_rand: i64, max_rand: i64) -> i64 {
        self.seed = (self.seed * 134775813 + 1) % 4294967296;
        self.seed % (max_rand - min_rand + 1) + min_rand
    }
}

fn main() {
    let mut generator1 = RandGen::new(123);
    let a = generator1.gen_range(3, 15);
    let b = generator1.gen_range(3, 15);
    let c = generator1.gen_range(3, 15);

    let mut generator2 = RandGen::new(123);
    let a2 = generator2.gen_range(3, 15);
    let b2 = generator2.gen_range(3, 15);
    let c2 = generator2.gen_range(3, 15);

    println!("1: {}", a == a2);
    println!("2: {}", b == b2);
    println!("3: {}", c == c2);

    println!("4: {}", a >= 3);
    println!("5: {}", b >= 3);
    println!("6: {}", c >= 3);

    println!("7: {}", a <= 15);
    println!("8: {}", b <= 15);
    println!("9: {}", c <= 15);

    // 9 x true w wyniku
}

// zad2

struct RandGen {
    seed: i64
}

impl RandGen {
    fn new(seed: i64) -> Self {
        Self { seed }
    }

    fn gen_range(&mut self, min_rand: i64, max_rand: i64) -> i64 {
        self.seed = (self.seed * 134775813 + 1) % 4294967296;
        self.seed % (max_rand - min_rand + 1) + min_rand
    }
}

struct Urna {
    znaki: Vec<char>,
    generator: RandGen,
}

impl Urna {
    fn new(generator: RandGen) -> Self {
        Self { 
            znaki: Vec::new(),
            generator
        }
    }

    fn losuj_z_us(&mut self) -> Option<char> { // losuj z usuwaniem
        self.znaki.pop()
    }

    fn losuj_bez_us(&self) -> Option<char> { // losuj bez usuwania
        self.znaki.last().copied()
    }

    fn doloz(&mut self, znak: char) {
        self.znaki.push(znak);
    }

    fn rozmiar(&self) -> usize {
        self.znaki.len()
    }
}

fn main() {
    let mut urna = Urna::new(RandGen::new(123));

    let a: Option<char> = urna.losuj_z_us();
    println!("1: {:?}", a.is_none());
    let a: Option<char> = urna.losuj_bez_us();
    println!("2: {:?}", a.is_none());


    urna.doloz('a');
    urna.doloz('b');
    urna.doloz('c');
    urna.doloz('d');

    println!("3: {:?}", urna.rozmiar() == 4);
    let y: Option<char> = urna.losuj_z_us();
    println!("4: {:?}", y.is_some());
    println!("5: {:?}", urna.rozmiar() == 3);
    let x: Option<char> = urna.losuj_bez_us();
    println!("6: {:?}", x.is_some());
    println!("7: {:?}", urna.rozmiar() == 3);
    println!("8: {:?}", x != y);
    urna.losuj_z_us();
    println!("9: {:?}", urna.rozmiar() == 2);
    urna.losuj_z_us();
    println!("10: {:?}", urna.rozmiar() == 1);
    urna.losuj_z_us();
    println!("11: {:?}", urna.rozmiar() == 0);
    let z: Option<char> = urna.losuj_z_us();
    println!("12: {:?}", z.is_none());
    println!("13: {:?}", urna.rozmiar() == 0);
}