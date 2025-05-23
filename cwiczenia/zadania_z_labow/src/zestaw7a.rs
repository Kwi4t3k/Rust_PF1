#[derive(PartialEq, Debug)]
struct Rgb {
    r: f32,
    g: f32,
    b: f32,
}

impl Rgb {
    fn from_3u8(r: u8, g: u8, b: u8) -> Self {
        Self {
            r: r as f32,
            g: g as f32,
            b: b as f32,
        }
    }

    fn from_3percent(r: f32, g: f32, b: f32) -> Option<Self> {
        let r1 = (r / 100.0 * 255.0) as u8;
        let g1 = (g / 100.0 * 255.0) as u8;
        let b1 = (b / 100.0 * 255.0) as u8;

        Some(Rgb::from_3u8(r1, g1, b1))
    }

    fn gray(procent: f32) -> Option<Self> {
        Some(Rgb::from_3percent(procent, procent, procent)?)
    }

    fn white() -> Self {
        Self::from_3u8(255, 255,255)
    }

    fn black() -> Self {
        Self::from_3u8(0, 0, 0)
    }

    fn invert(&mut self) {
        self.r = 255.0 - self.r;
        self.g = 255.0 - self.g;
        self.b = 255.0 - self.b;
    }

    fn intensity(&self) -> f32 {
        (self.r + self.g + self.b) / (3.0 * 255.0)
    }

    fn as_rgb_u8tuple(&self) -> (u8, u8, u8) {
        (self.r as u8, self.g as u8, self.b as u8)
    }

    fn as_cmy_u8tuple(&self) -> (u8, u8, u8) {
        ((255.0 - self.r) as u8, (255.0 - self.g) as u8, (255.0 - self.b) as u8)
    }
}

fn main() {
    let szary1 = Rgb::from_3u8(127, 127, 127);
    let szary2 = Rgb::from_3percent(50.0, 50.0, 50.0).unwrap();
    let szary3 = Rgb::gray(50.0).unwrap();
    let fiolet = Rgb::from_3u8(100, 35, 120);
    let bialy1 = Rgb::white();
    let bialy2 = Rgb::from_3u8(255, 255, 255);
    let mut czarny1 = Rgb::black();
    let czarny2 = Rgb::from_3u8(0, 0, 0);

    println!("{} {}", szary1 == szary2, szary1 == szary3);
    println!("{} {}", bialy1 == bialy2, czarny1 == czarny2);

    czarny1.invert();

    println!("{}", bialy1 == czarny1);
    println!("{}", fiolet.intensity() == 1.0 / 3.0);
    println!("{}", fiolet.as_rgb_u8tuple() == (100, 35, 120));
    println!("{}", fiolet.as_cmy_u8tuple() == (155, 220, 135));
}