use crate::randgen::RandGen;

pub struct Urna<T> {
    znaki: Vec<T>,
    generator: RandGen,
}

impl <T: Copy> Urna<T> {
    pub fn new(generator: RandGen) -> Self {
        Self {
            znaki: Vec::new(),
            generator,
        }
    }

    pub fn losuj_z_us(&mut self) -> Option<T> {
        self.znaki.pop()
    }

    pub fn losuj_bez_us(&mut self) -> Option<T> {
        self.znaki.last().copied()
    }

    pub fn doloz(&mut self, znak: T) {
        self.znaki.push(znak)
    }

    pub fn rozmiar(&self) -> usize {
        self.znaki.len()
    }
}