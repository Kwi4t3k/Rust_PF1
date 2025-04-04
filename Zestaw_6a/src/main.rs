fn indeksy_iterator(tablica: Vec<String>, element: &str) -> Vec<usize> {
    tablica.into_iter()
        .enumerate()  // Otrzymujemy (index, value)
        .filter(|(_, x)| x == element)  // Porównujemy wartość z elementem
        .map(|(i, _)| i)  // Bierzemy tylko indeksy
        .collect()  // Zbieramy wyniki do wektora
}

fn indeksy_petla(tablica: Vec<String>, element: &str) -> Vec<usize> {
	let mut result = Vec::new();

	for (i, s) in tablica.iter().enumerate() { // `enumerate()` daje dostęp do indeksu i elementu
		if s == element { // tylko jeśli element jest równy szukanemu napisowi
			result.push(i);
		}
	}
	
	result
}

fn main() {
    let test_strings = vec![
		String::from("kot"),
		String::from("pies"),
		String::from("dom"),
		String::from("Ala"),
		String::from("zamek"),
		String::from("król"),
		String::from("robot"),
		String::from("1234"),
		String::from("test1"),
		String::from("inny"),
		String::from("pizza"),
		String::from("brutto"),
		String::from("lekki"),
		String::from("dzienny"),
		String::from("programowanie"),
		String::from("Rust"),
		String::from("wow"),
		String::from("gamma"),
		String::from("delta"),
		String::from("epsilon"),
		String::from("hello123"),
		String::from("abcd"),
		String::from("xyz"),
		String::from("foo"),
		String::from("bar"),
		String::from("brutto"),
	];

    println!("{:?}", indeksy_iterator(test_strings.clone(), "brutto"));
    println!("{:?}", indeksy_petla(test_strings, "brutto"));
}