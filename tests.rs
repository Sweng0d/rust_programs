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
        self.balance += amount
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
}
