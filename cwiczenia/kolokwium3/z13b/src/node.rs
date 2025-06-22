#[derive(Debug)]
pub struct Node<T> {
    wartosc: T,
    poprzedni: Option<Box<Node<T>>>,
}

impl<T> Node<T> {
    pub fn new(wartosc: T, poprzedni: Option<Box<Node<T>>>) -> Box<Self> {
        Box::new(
            Self { 
                wartosc, 
                poprzedni,
            }
        )
    }
    
    pub fn poprzedni(self) -> Option<Box<Node<T>>> {
        self.poprzedni
    }
    
    pub fn wartosc(&self) -> &T {
        &self.wartosc
    }
}
