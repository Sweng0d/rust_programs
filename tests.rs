pub struct SavingsAccount {
    balance: i32,

}

impl SavingsAccount {
    pub fn new() -> SavingsAccount{
        SavingsAccount{
            balance: 0,
        }
    }

    pub fn get_balance(&self) -> i32 {
        self.balance
    }

    pub fn deposit(&mut self, amount: i32) {
        if amount < 0 {
            panic!("Cannot deposit a negative value");
        }
        self.balance += amount
    }

    pub fn transfer (from: &mut Self, to: &mut Self, amount: i32) {
        from.balance -= amount;
        to.balance += amount;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_have_a_starting_balance_of_zero() {
        let account: SavingsAccount = SavingsAccount::new();
        assert_eq!(account.get_balance(), 0);
    }

    #[test]
    fn deposit_is_right() {
        let mut account = SavingsAccount::new();
        account.deposit(150);
        assert_eq!(account.get_balance(), 150);
        assert_ne!(account.get_balance(), 0);
        assert!(account.get_balance() == 150)
    }

    #[test]
    fn transfer_working() {
        let mut account1 = SavingsAccount::new();
        let mut account2 = SavingsAccount::new();
        account1.deposit(200);
        assert_eq!(account1.get_balance(), 200);

        SavingsAccount::transfer(&mut account1, &mut account2, 50);
        assert_eq!(account1.get_balance(), 150);
        assert_eq!(account2.get_balance(), 50);
    }

    #[test]
    #[should_panic]
    fn should_panic_if_deposit_is_negative() {
        let mut account1 = SavingsAccount::new();
        account1.deposit(-10);

    }
}
