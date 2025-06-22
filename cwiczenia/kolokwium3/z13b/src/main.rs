use z13b::stos::Stos;

fn main() {
    let mut stos = Stos::new();
    println!("{:?}", stos.pop());

    stos.push(10);
    stos.push(20);

    println!("{:?}", stos);
    println!("{:?}", stos.top());
    stos.pop();
    println!("{:?}", stos);
    println!("{:?}", stos.top());
    println!("{:?}", stos.is_empty());
    stos.pop();
    println!("{:?}", stos.is_empty());
}