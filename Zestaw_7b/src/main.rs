struct Matrix {
    wysokosc: f64,
    szerokosc: f64,
    dane: Vec<Vec<i32>>,
}

impl Matrix {
    fn new(wysokosc: f64, szerokosc: f64, wypelniacz: i32) -> Self {
        let mut dane = Vec::new();

        for _ in 0..wysokosc as usize {
            let mut wiersz = Vec::new();

            for _ in 0..szerokosc as usize {
                wiersz.push(wypelniacz);
            }
            dane.push(wiersz);
        }

        Self {
            wysokosc,
            szerokosc,
            dane,
        }
    }

    fn zerowa(wysokosc: f64, szerokosc: f64) -> Self {
        Matrix::new(szerokosc, wysokosc, 0)
    }

    fn from_vec(wysokosc: f64, szerokosc: f64, dane: Vec<Vec<i32>>) -> Self {
        Self {
            wysokosc,
            szerokosc,
            dane,
        }
    }

    fn jednostkowa(wysokosc: f64) -> Self {
        let mut wypelniacz = Vec::new();

        for i in 0..wysokosc as usize {
            let mut wiersz = Vec::new();

            for j in 0..wysokosc as usize {
                if i == j {
                    wiersz.push(1);
                } else {
                    wiersz.push(0);
                }
            }
            wypelniacz.push(wiersz);
        }

        Matrix::from_vec(wysokosc, wysokosc, wypelniacz)
    }

    fn element(&self, indeks_wiersza: usize, indeks_kolumny: usize) -> f64 {
        self.dane[indeks_wiersza][indeks_kolumny] as f64
    }

    fn zmien_element(&mut self, indeks_wiersza: usize, indeks_kolumny: usize, nowa_wartosc: i32) {
        self.dane[indeks_wiersza][indeks_kolumny] = nowa_wartosc
    }

    fn suma(macierz1: Matrix, macierz2: Matrix) -> Option<Matrix> {
        if macierz1.wysokosc == macierz2.wysokosc && macierz1.szerokosc == macierz2.szerokosc {
            let mut wynik = Vec::new();

            for i in 0..macierz1.wysokosc as usize {
                let mut wiersz = Vec::new();

                for j in 0..macierz1.szerokosc as usize {
                    let suma_elementow = macierz1.dane[i][j] + macierz2.dane[i][j];
                    wiersz.push(suma_elementow);
                }
                wynik.push(wiersz);
            }

            Some(Matrix {
                wysokosc: macierz1.wysokosc,
                szerokosc: macierz1.szerokosc,
                dane: wynik })
        } else {
            None
        }
    }

    fn wyswietl(&self) {
        for i in 0..self.wysokosc as usize {
            for j in 0..self.szerokosc as usize {
                print!("{} ", self.dane[i][j]);
            }
            println!();
        }
    }
}

fn main() {
    Matrix::wyswietl(&Matrix::new(2.0, 5.0, 5));

    println!("---------------------------");

    Matrix::wyswietl(&Matrix::zerowa(5.0, 4.0));

    println!("---------------------------");

    Matrix::wyswietl(&Matrix::jednostkowa(5.0));

    println!("---------------------------");

    let matrix = Matrix::jednostkowa(4.0);
    Matrix::wyswietl(&matrix);
    println!("Element [0, 0] z macierzy: {}", matrix.element(0, 0)); // 1
    println!("Element [1, 1] z macierzy: {}", matrix.element(1, 1)); // 1
    println!("Element [0, 1] z macierzy: {}", matrix.element(0, 1)); // 0

    println!("---------------------------");

    let mut matrix = Matrix::jednostkowa(4.0);
    Matrix::wyswietl(&matrix);

    matrix.zmien_element(0, 1, 5);

    println!();
    Matrix::wyswietl(&matrix);

    println!("---------------------------");

    let matrix1 = Matrix::jednostkowa(4.0);
    let matrix2 = Matrix::jednostkowa(4.0);

    let suma = Matrix::suma(matrix1, matrix2);

    Matrix::wyswietl(&suma.unwrap());
}