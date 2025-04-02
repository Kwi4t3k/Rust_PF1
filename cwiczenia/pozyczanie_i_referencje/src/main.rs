// Understanding references
// References: Enable you to borrow values without taking ownership
// Immutable reference
// Mutable reference
// Create Reference by add `&`
// -I- Immutable reference

// fn main() {
//     let x: i32 = 5;
//     // let r = x;    <- jeśli tak to `x` przestaje istnieć i ownership jest na `r`
//     let r = &x;  // <- `r` is a `&` reference, so the data it refers to cannot be written
//     *r += 1;
//
//     println!("Value of x: {x}");
//     println!("Value of r: {r}");
//
//
//     // Only one mutable reference to a value or any number of immutable references
//     let mut x: i32 = 5;
//     let r = &mut x;
//     *r += 1;
//     *r -= 3;
//
//     println!("Value of x: {}", x); // Value of x: 3 | 5 + 1 = 6 - 3 = 3
// }

// -------------------------------------------------------------------------------
// STRUCT
fn main() {
    let mut account = BankAccount {
        owner: "Alice".to_string(),
        balance: 150.55,
    };

    // Immutable borrow to check the balance
    account.check_balance();

    // Mutable borrow to withdraw the money
    account.withdraw(45.5);

    // Immutable borrow to check the balance
    account.check_balance();
}

struct BankAccount {
    owner: String,
    balance: f64,
}

impl BankAccount {
    fn withdraw(&mut self, amount: f64) {
        println!("Withdrawing {} $ from account owned by {}", amount, self.owner);
        self.balance -= amount;
    }

    fn check_balance(&self) {
        println!("Account owned by {} has a balance of {:.2} $", self.owner, self.balance);
    }
}