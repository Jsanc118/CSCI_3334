mod bank_account;

use bank_account::BankAccount;

fn main() {
    let mut account = BankAccount::new(100.0);
    println!("Initial balance: ${}", account.balance());

    account.deposit(50.0);
    println!("After depositing $50: ${}", account.balance());

    account.withdraw(30.0);
    println!("After withdrawing $30: ${}", account.balance());

    // Attempt to withdraw a negative amount
    account.withdraw(-20.0);
    println!("After attempting to withdraw -$20: ${}", account.balance());

    // Attempt to withdraw more than the balance
    account.withdraw(200.0);
    println!("After attempting to withdraw $200: ${}", account.balance());

    // Bonus Challenge: Apply interest
    account.apply_interest(0.05); // Apply 5% interest
    println!("After applying 5% interest: ${}", account.balance());
}