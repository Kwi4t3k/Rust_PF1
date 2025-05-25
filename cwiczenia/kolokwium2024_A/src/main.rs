use std::cmp::Ordering;

// 1
fn email(email: &str) -> bool {
    if email.is_empty() {
        return false;
    }

    let mut liczba_malp = 0;
    for c in email.chars() {
        if c == '@' {
            liczba_malp += 1;
        }

        if !(c.is_alphanumeric() || c == '@' || c == '.') {
            return false;
        }
    }

    if liczba_malp != 1 {
        return false;
    }

    let pierwszy_znak = email.chars().next().unwrap();
    let ostatni_znak = email.chars().rev().next().unwrap();
    if !pierwszy_znak.is_alphanumeric() || !ostatni_znak.is_alphanumeric() {
        return false;
    }

    let czesci: Vec<_> = email.split('@').collect();
    let domena = czesci[1];
    if !domena.contains('.') {
        return false;
    }

    true
}

// 2

struct Set {
    zbior: Vec<u32>
}

impl Set {
    fn new() -> Self {
        Self {
            zbior: Vec::new(),
        }
    }

    fn from_slice(slice: &[u32]) -> Set {
        let mut tmp = slice.to_vec();

        tmp.sort_unstable();
        tmp.dedup();

        Set {
            zbior: tmp
        }
    }

    fn union(set1: &Set, set2: &Set) -> Set {
        let mut tmp = Vec::new();

        for &elem in set1.zbior.iter() {
            if !tmp.contains(&elem) {
                tmp.push(elem);
            }
        }

        for &elem in set2.zbior.iter() {
            if !tmp.contains(&elem) {
                tmp.push(elem);
            }
        }

        tmp.sort();

        Self {
            zbior: tmp
        }
    }
}

impl PartialEq for Set {
    fn eq(&self, other: &Self) -> bool {
        self.zbior == other.zbior
    }
}

impl Eq for Set {}

impl PartialOrd for Set {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        // 1) Jesli zbiory sa identyczne, mamy Ordering::Equal
        if self.zbior == other.zbior {
            return Some(Ordering::Equal);
        }
        // 2) Sprawdzamy, czy kazdy element self znajduje sie w other
        if self.zbior.iter().all(|x| other.zbior.contains(x)) {
            // 2a) Jesli dodatkowo self ma mniej elementow niz other, to proper subset
            if self.zbior.len() < other.zbior.len() {
                return Some(Ordering::Less);
            }
        }
        // 3) Sprawdzamy odwrotnie: czy kazdy element other znajduje sie w self
        if other.zbior.iter().all(|x| self.zbior.contains(x)) {
            // 3a) Jesli other jest mniejszy, to self jest proper superset
            if other.zbior.len() < self.zbior.len() {
                return Some(Ordering::Greater);
            }
        }
        // 4) W przeciwnym wypadku zbiory nie sa porownywalne
        None
    }
}

// 3
#[derive(Debug, Clone)]
struct Transaction {
    kwota_przelewu: f32,
    numer_konta_nadawcy: String,
    numer_konta_odbiorcy: String,
}

impl Transaction {
    fn new(kwota: f32, nadawca: &str, odbiorca: &str) -> Self {
        Self {
            kwota_przelewu: kwota,
            numer_konta_nadawcy: nadawca.to_string(),
            numer_konta_odbiorcy: odbiorca.to_string(),
        }
    }
}

struct  BankAccount {
    numer_konta: String,
    historia_transakcji: Vec<Transaction>,
}

impl BankAccount {
    fn new(numer: &str) -> Self {
        Self {
            numer_konta: numer.to_string(),
            historia_transakcji: Vec::new(),
        }
    }

    fn transaction(&mut self, t: Transaction) {
        if self.numer_konta == t.numer_konta_odbiorcy || self.numer_konta == t.numer_konta_nadawcy {
            self.historia_transakcji.push(t);
        }
    }

    fn balance(&self) -> f32 {
        let mut stan_konta = 0.0;
        for transakcja in &self.historia_transakcji {

            if self.numer_konta == transakcja.numer_konta_odbiorcy  {
                stan_konta += transakcja.kwota_przelewu;
            }
            else if self.numer_konta == transakcja.numer_konta_nadawcy {
                stan_konta -= transakcja.kwota_przelewu;
            }
        }
        stan_konta
    }

    fn last(&self) -> Option<Transaction> {
        self.historia_transakcji.last().cloned()
    }
}

fn main() {
    // --- test Zadanie 1 ---
    println!("Zadanie1:");
    println!("alan@example.com -> {}", email("alan@example.com")); // true
    println!(".ala@ex.pl -> {}",     email(".ala@ex.pl"));         // false
    println!("ala@ex -> {}",         email("ala@ex"));             // false
    println!("ala@@ex.pl -> {}",     email("ala@@ex.pl"));         // false

    // --- test Zadanie 2 ---
    println!("\nZadanie2:");
    let s1 = Set::from_slice(&[1, 3, 5, 3]);           // {1,3,5}
    let s2 = Set::from_slice(&[2, 3, 4]);              // {2,3,4}
    let su = Set::union(&s1, &s2);                     // {1,2,3,4,5}
    println!("union {{1,3,5}} i {{2,3,4}} = {:?}", su.zbior);
    println!("s1 < su ? {:?}", s1 < su);                 // true
    println!("su > s2 ? {:?}", su > s2);                 // true
    println!("s1 cmp s2 ? {:?}", s1.partial_cmp(&s2));   // None

    // --- test Zadanie 3 ---
    println!("\nZadanie3:");
    // utworz konta
    let mut konto1 = BankAccount::new("111");
    let mut konto2 = BankAccount::new("222");
    // transakcje
    let t1 = Transaction::new(100.0, "111", "222");
    let t2 = Transaction::new( 30.0, "222", "111");
    let t3 = Transaction::new( 50.0, "333", "111"); // inny nadawca

    // dodajemy do historii
    konto1.transaction(t1.clone());
    konto1.transaction(t2.clone());
    konto1.transaction(t3.clone());
    konto2.transaction(t1);
    konto2.transaction(t2);
    konto2.transaction(t3);

    // sprawdzamy saldo i ostatnia
    println!("konto1 saldo = {}", konto1.balance());     // 100 - ? + ? = 100 -100 +30 +50 = -20?
    println!("konto2 saldo = {}", konto2.balance());     // 100? -30? = 70?
    println!("konto1 ostatnia = {:?}", konto1.last());
}