#[derive(PartialEq, Debug)] // automatycznie implementuje porównywanie i debugowanie struktury
struct Rgb {
    x: f32, // składowa czerwona (r)
    y: f32, // składowa zielona (g)
    z: f32, // składowa niebieska (b)
}

impl Rgb {
    fn from_3u8(r: u8, g: u8, b: u8) -> Self {
        Self {
            x: r as f32, // konwersja z u8 na f32
            y: g as f32,
            z: b as f32,
        }
    }

    fn from_3percent(r: f32, g: f32, b: f32) -> Option<Self> {
        // zamiana wartości procentowych na 0–255
        let x1 = (r / 100.0 * 255.0) as u8;
        let y1 = (g / 100.0 * 255.0) as u8;
        let z1 = (b / 100.0 * 255.0) as u8;

        let result = Rgb::from_3u8(x1, y1, z1);

        Some(result) // zwracamy wynik jako Some
    }

    fn gray(procent: f32) -> Option<Self> {
        Self::from_3percent(procent, procent, procent) // szarość = wszystkie składowe równe
    }

    fn white() -> Self {
        // Self {
        //     x: 255.0,
        //     y: 255.0,
        //     z: 255.0,
        // }

        Self::from_3u8(255, 255, 255) // biały to (255, 255, 255)
    }

    fn black() -> Self {
        // Self {
        //     x: 0.0,
        //     y: 0.0,
        //     z: 0.0,
        // }

        Self::from_3u8(0, 0, 0) // czarny to (0, 0, 0)
    }

    fn invert(&mut self) {
        self.x = 255.0 - self.x; // odwracamy składową x
        self.y = 255.0 - self.y; // odwracamy składową y
        self.z = 255.0 - self.z; // odwracamy składową z

        // if self.x - 255.0 < 0.0 {
        //     let a = self.x - 255.0;
        //     self.x = a * -1.0;
        // } else {
        //     self.x = 0.0;
        // }
        //
        // if self.y - 255.0 < 0.0 {
        //     let a = self.y - 255.0;
        //     self.y = a * -1.0;
        // } else {
        //     self.y = 0.0;
        // }
        //
        // if self.z - 255.0 < 0.0 {
        //     let a = self.z - 255.0;
        //     self.z = a * -1.0;
        // } else {
        //     self.z = 0.0;
        // }
    }

    // intensity - sumujemy wszystko i dzielimy przez maksima
    fn intensity(&self) -> f32 {
        (self.x + self.y + self.z) / (3.0 * 255.0) // uśredniona jasność koloru (skala 0–1)
    }

    // as_rgb_u8tuple - zwaraca krotkę 3 elementową u8 zwraca kolor
    fn as_rgb_u8tuple(&self) -> (u8, u8, u8) {
        (self.x as u8, self.y as u8, self.z as u8) // rzutujemy składowe f32 z powrotem na u8
    }

    // as_cmy_u8tuple
    fn as_cmy_u8tuple(&self) -> (u8, u8, u8) {
        // odwrotność koloru w przestrzeni CMY (C = 255 - R itd.)
        ((255.0 - self.x) as u8, (255.0 - self.y) as u8, (255.0 - self.z) as u8)
    }
}

fn main() {
    let szary1 = Rgb::from_3u8(127, 127, 127); // kolor szary w RGB
    let szary2 = Rgb::from_3percent(50.0, 50.0, 50.0).unwrap(); // kolor szary z wartości procentowych
    let szary3 = Rgb::gray(50.0).unwrap(); // skrócona wersja tworzenia szarości
    let fiolet = Rgb::from_3u8(100, 35, 120); // przykładowy kolor
    let bialy1 = Rgb::white(); // tworzy biały
    let bialy2 = Rgb::from_3u8(255, 255, 255); // też biały
    let mut czarny1 = Rgb::black(); // tworzy czarny
    let czarny2 = Rgb::from_3u8(0, 0, 0); // też czarny

    println!("{} {}", szary1 == szary2, szary1 == szary3); // porównanie trzech wersji szarości
    println!("{} {}", bialy1 == bialy2, czarny1 == czarny2); // sprawdzenie równości białych i czarnych
    czarny1.invert(); // czarny zamieniony na biały
    println!("{}", bialy1 == czarny1); // porównanie z białym

    println!("{}", fiolet.intensity() == 1.0/3.0); // sprawdzenie intensywności (średnia wartość RGB)
    println!("{}", fiolet.as_rgb_u8tuple() == (100, 35, 120)); // RGB jako krotka
    println!("{}", fiolet.as_cmy_u8tuple() == (155, 220, 135)); // CMY jako krotka

    /*let mut czarny1 = Rgb::black();
    println!("czarny: {:?}", czarny1);

    let bialy1 = Rgb::white();
    println!("biały: {:?}", bialy1);

    czarny1.invert();
    println!("czarny invert: {:?}", czarny1);

    let mut szary1 = Rgb::from_3u8(127, 127, 127);
    println!("szary: {:?}", szary1);
    szary1.invert();
    println!("szary invert: {:?}", szary1);

    /*let szary2 = Rgb::gray(50.0).unwrap();
    println!("{:?}", szary2); */

    let szary2 = Rgb::from_3percent(50.0, 50.0, 50.0).unwrap();
    println!("szary2 {:?}", szary2); */
}