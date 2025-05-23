struct Matrix {
    h: usize, // wysokość
    w: usize, // szerokość
    d: Vec<Vec<f64>> // dane
}

impl Matrix {
    fn new(wysokosc: usize, szerokosc: usize, wypelniacz: f64) -> Self {
        let mut dane = Vec::new();

        for _ in 0..wysokosc {
            let mut wiersz = Vec::new();

            for _ in 0..szerokosc {
                wiersz.push(wypelniacz);
            }
            dane.push(wiersz)
        }

        Self {
            h: wysokosc,
            w: szerokosc,
            d: dane,
        }
    }

    fn zerowa(wysokosc: usize, szerokosc: usize) -> Self {
        Matrix::new(wysokosc, szerokosc, 0.0)
    }

    fn jednostkowa(wysokosc: usize) -> Self {
        let mut dane = Vec::new();

        for i in 0..wysokosc {
            let mut wiersz = Vec::new();

            for j in 0..wysokosc {
                if i == j {
                    wiersz.push(1.0);
                } else {
                    wiersz.push(0.0);
                }
            }
            dane.push(wiersz)
        }

        Self {
            h: wysokosc,
            w: wysokosc,
            d: dane,
        }
    }

    fn element(&mut self, indeks_wiersza: usize, indeks_kolumny: usize) -> f64 {
        self.d[indeks_wiersza][indeks_kolumny]
    }

    fn zmien_element(&mut self, indeks_wiersza: usize, indeks_kolumny: usize, nowa_wartosc: f64) {
        self.d[indeks_wiersza][indeks_kolumny] = nowa_wartosc
    }

    fn suma(macierz1: &Matrix, macierz2: &Matrix) -> Option<Matrix> {
        if macierz1.w == macierz2.w && macierz1.h == macierz2.h {
            let mut dane = Vec::new();

            for i in 0..macierz1.h {
                let mut wiersz = Vec::new();

                for j in 0..macierz1.w {
                    wiersz.push(macierz1.d[i][j] + macierz2.d[i][j]);
                }
                dane.push(wiersz)
            }

            Some(Self {
                h: macierz1.h,
                w: macierz1.w,
                d: dane,
            })
        } else {
            None
        }
    }

    fn wyswietl(&self) {
        for i in 0..self.h {
            for j in 0..self.w {
                print!("{} ", self.d[i][j]);
            }
            println!();
        }
    }
}

fn main() {
    let mut m1 = Matrix::new(3, 3, 2.0);
    println!("Macierz m1:");
    m1.wyswietl();

    m1.zmien_element(1, 1, 5.0);
    println!("Po zmianie elementu [1,1]:");
    m1.wyswietl();

    let m2 = Matrix::zerowa(3, 3);
    println!("Macierz zerowa m2:");
    m2.wyswietl();

    let m3 = Matrix::jednostkowa(3);
    println!("Macierz jednostkowa m3:");
    m3.wyswietl();

    match Matrix::suma(&m1, &m3) {
        Some(suma) => {
            println!("Suma m1 + m3:");
            suma.wyswietl();
        }
        None => println!("Macierze mają różne wymiary, nie można zsumować."),
    }

    let el = m1.element(1, 1);
    println!("Element [1,1] macierzy m1: {}", el);
}
