#[derive(Debug, PartialEq)]
pub struct User {
    name: String,
    credit_line: u64,
    // balance: BalanceAmount,
    balance: i64,
}

#[allow(dead_code)]
pub struct BalanceAmount {
    value: i64,
}

// newtype pattern - style of programming
// possible with zero-cost abstraction
// use transparent attribute

// impl BalanceAmount {
//     fn is_credit(&self) -> bool{
//         self.value < 0
//     }
//
//     fn is_debit(&self) -> bool {
//         self.value >= 0
//     }
// }

// inherent methods - not tied to any trait
// In double-entry accounting — credits record outgoing money
fn is_credit(value: &i64) -> bool {
    value < &0
}

// inherent methods - not tied to any trait
// In double-entry accounting — debits record incoming money
fn is_debit(value: &i64) -> bool {
    value >= &0
}

trait BalanceAccessor {
    fn get_balance(&self) -> i64;
    fn set_balance(&mut self, value: i64);
}

impl BalanceAccessor for User {
    fn get_balance(&self) -> i64 {
        self.balance
    }

    fn set_balance(&mut self, value: i64) {
        self.balance = value
    }
}

impl User {
    // type of name?
    pub fn new(name: String, credit_line: u64, balance: i64) -> Self {
        Self {
            name,
            credit_line,
            balance,
        }
    }
}

impl MaxCredit for User {
    fn max_credit(&self) -> u64 {
        <i64 as TryInto<u64>>::try_into(self.balance).unwrap() + self.credit_line
    }
}

trait Balance {
    fn balance(&self) -> BalanceSheet;
}

trait MaxCredit {
    fn max_credit(&self) -> u64;
}

#[derive(Debug, PartialEq)]
pub struct Bank {
    users: Vec<User>,
    name: String,
    credit_interest: u64,
    debit_interest: u64,
}

impl Bank {
    pub(crate) fn merge(&mut self, another: Bank) -> bool {
        for user_from_new_bank in another.users {
            let user_name = &user_from_new_bank.name;
            match self.get_user_index_by_id(user_name) {
                None => self.users.push(user_from_new_bank),
                Some(overlapping_user_index) => {
                    let old_balance = self.users[overlapping_user_index].balance;
                    self.users[overlapping_user_index].balance =
                        old_balance + user_from_new_bank.balance
                }
            }
        }
        true
    }
}

pub struct BalanceSheet {
    liabilities: u64,
    assets: i64,
}

impl Balance for Bank {
    fn balance(&self) -> BalanceSheet {
        let total_liabilities: i64 = self
            .users
            .iter()
            .filter(|user| is_credit(&user.balance))
            .map(|user| user.balance)
            .sum();

        let total_assets: i64 = self
            .users
            .iter()
            .filter(|user| is_debit(&user.balance))
            .map(|user| user.balance)
            .sum();

        let to: u64 = (-total_liabilities).try_into().unwrap();
        BalanceSheet {
            assets: total_assets,
            liabilities: to,
        }
    }
}

#[derive(PartialEq, Debug)]
pub enum TransferFundsError {
    InvalidAmount,
    OriginUserNotFound,
    DestinationUserNotFound,
    OriginUserNotAllowed,
}

impl Bank {
    // type of users?
    pub fn new(users: Vec<User>, name: String, credit_interest: u64, debit_interest: u64) -> Self {
        Self {
            users,
            name,
            credit_interest,
            debit_interest,
        }
    }
    pub(crate) fn get_user_by_id(&self, user_id: String) -> Option<&User> {
        self.users.iter().find(|&user| user.name == user_id)
    }
    fn get_user_index_by_id(&self, user_id: &str) -> Option<usize> {
        self.users.iter().position(|user| user.name == user_id)
    }
    pub(crate) fn transfer_funds(
        &mut self,
        origin_username: &str,
        destination_username: &str,
        amount: u64,
    ) -> Result<(), TransferFundsError> {
        if amount == 0 {
            return Err(TransferFundsError::InvalidAmount);
        }
        // let users_iter_mut = self.users.iter_mut();
        // let both_users_exist = users_iter_mut.filter(|user| user.name == origin_username || user.name == destination_username).count() == 2;
        let maybe_origin_index = self.get_user_index_by_id(origin_username);
        let maybe_destination_index = self.get_user_index_by_id(destination_username);

        // let both_users_exist = users_iter_mut.filter(|user| user.name == origin_username || user.name == destination_username).count() == 2;
        // first_user_exists = users_iter_mut(|user| user.name == origin_username).count() == 1;
        // let users_iter_mut = self.users.iter_mut();
        // let second_user_exists = users_iter_mut.filter(|user| user.name == destination_username).count() == 1;

        // let mut first_user = self.users[first_user_exists_index as usize];
        // first_user.set_balance(first_user.get_balance() - amount);
        // self.users[second_user_exists_index as usize].set_balance(self.users[second_user_exists_index as usize].get_balance() + amount);

        let (origin_index, destination_index) = match (maybe_origin_index, maybe_destination_index)
        {
            (Some(origin_index), Some(destination_index)) => (origin_index, destination_index),
            (None, _) => {
                return Err(TransferFundsError::OriginUserNotFound);
            }
            (_, None) => {
                return Err(TransferFundsError::DestinationUserNotFound);
            }
        };

        let has_credit_limit = self.users[origin_index].max_credit() >= amount;
        if !has_credit_limit {
            return Err(TransferFundsError::OriginUserNotAllowed);
        }

        let amount_as_i64: i64 = amount.try_into().unwrap();
        {
            let origin_balance = self.users[origin_index].get_balance() - amount_as_i64;
            self.users[origin_index].set_balance(origin_balance);
        }
        {
            let destination_balance = self.users[destination_index].get_balance() + amount_as_i64;
            self.users[destination_index].set_balance(destination_balance);
        }

        // self.users[first_user_index.unwrap()]
        // second_user.map(|mut user| user.set_balance(user.get_balance()));

        Ok(())
    }

    pub(crate) fn accrue_interest(&mut self) -> bool {
        for user in self.users.iter_mut() {
            let selected_interest = if is_debit(&user.balance) {
                self.debit_interest
            } else {
                self.credit_interest
            };
            let new_balance = user.balance
                * (<u64 as TryInto<i64>>::try_into(selected_interest).unwrap())
                / 10_000;
            user.balance += new_balance;
        }
        true
    }
}

#[cfg(test)]
mod tests_user {
    use super::*;

    #[test]
    fn assign_the_fields() {
        let user = User::new("John".to_string(), 1, 1);

        assert_eq!(user.name, "John".to_string());
        assert_eq!(user.credit_line, 1);
        assert_eq!(user.balance, 1);
    }

    #[test]
    fn equality_of_the_object_includes_equality_of_all_fields() {
        let user1 = User::new("John".to_string(), 1, 1);
        let user2 = User::new("John".to_string(), 1, 1);

        assert_eq!(user1, user2);
    }
}

#[cfg(test)]
mod tests_bank {
    use super::*;

    #[test]
    fn assign_the_fields() {
        let user1 = User::new("John".to_string(), 1, 1);
        let bank = Bank::new(vec![user1], "First Bank".to_string(), 1, 4);

        assert_eq!(bank.users.len(), 1);
        assert_eq!(bank.name, "First Bank".to_string());
        assert_eq!(bank.credit_interest, 1);
        assert_eq!(bank.debit_interest, 4);
    }

    #[test]
    fn equality_of_the_object_includes_equality_of_all_fields() {
        let user1 = User::new("John".to_string(), 1, 1);
        let user2 = User::new("John".to_string(), 1, 1);
        let bank = Bank::new(vec![user1], "First Bank".to_string(), 1, 4);
        let bank2 = Bank::new(vec![user2], "First Bank".to_string(), 1, 4);

        assert_eq!(bank, bank2);
    }

    // - calc_balance: calculates bank’s balance sheet in the form of two numbers: total bank liabilities and assets (liabilities represent all debit accounts, assets represent all credit accounts).
    // – transfer_funds: accepts two user names and transfer amount as positive integer. Transfers the specified amount from one user to another. Returns an error, if its can not be done (e.g. if the origin user hit his credit limit).
    // – accrue_interest: update user balances according to bank’s interest rates on credit and debit.
    #[test]
    fn calculate_balance_of_the_bank() {
        let user1 = User::new("John1".to_string(), 100, 1);
        let user2 = User::new("John".to_string(), 1, 90);
        let bank = Bank::new(vec![user1, user2], "First Bank".to_string(), 1, 4);

        let total_balance = bank.balance();

        assert_eq!(total_balance.assets, 1 + 90);
        assert_eq!(total_balance.liabilities, 0)
    }

    #[test]
    fn calculate_balance_of_the_bank_when_there_are_credits() {
        let user1 = User::new("John1".to_string(), 100, -1);
        let user2 = User::new("John".to_string(), 1, 90);
        let bank = Bank::new(vec![user1, user2], "First Bank".to_string(), 1, 4);

        let total_balance = bank.balance();

        assert_eq!(total_balance.assets, 90);
        assert_eq!(total_balance.liabilities, 1);
    }

    #[test]
    fn transfer_funds_happy_path() {
        let user1 = User::new("user1".to_string(), 100, 1);
        let user2 = User::new("user2".to_string(), 1, 90);
        let mut bank = Bank::new(vec![user1, user2], "First Bank".to_string(), 1, 4);

        let result = bank.transfer_funds("user1", "user2", 2);

        assert_eq!(result, Ok(()));
        assert_eq!(
            bank.get_user_by_id("user1".to_string()).unwrap().balance,
            -1
        );
        assert_eq!(
            bank.get_user_by_id("user2".to_string()).unwrap().balance,
            92
        );
    }

    #[test]
    fn transfer_funds_credit_limit_exceeded() {
        let user1 = User::new("user1".to_string(), 0, 1);
        let user2 = User::new("user2".to_string(), 1, 90);
        let mut bank = Bank::new(vec![user1, user2], "First Bank".to_string(), 1, 4);

        let result = bank.transfer_funds("user1", "user2", 2);

        assert_eq!(result, Err(TransferFundsError::OriginUserNotAllowed));
        assert_eq!(bank.get_user_by_id("user1".to_string()).unwrap().balance, 1);
        assert_eq!(
            bank.get_user_by_id("user2".to_string()).unwrap().balance,
            90
        );
    }

    #[test]
    fn transfer_funds_origin_user_does_not_exist() {
        let user2 = User::new("user2".to_string(), 0, 1);
        let mut bank = Bank::new(vec![user2], "First Bank".to_string(), 1, 4);

        let result = bank.transfer_funds("NON_EXISTING", "user2", 2);

        assert_eq!(result, Err(TransferFundsError::OriginUserNotFound));
        assert_eq!(bank.get_user_by_id("user2".to_string()).unwrap().balance, 1);
    }

    #[test]
    fn transfer_funds_destination_user_does_not_exist() {
        let user2 = User::new("user2".to_string(), 0, 1);
        let mut bank = Bank::new(vec![user2], "First Bank".to_string(), 1, 4);

        let result = bank.transfer_funds("user2", "NON_EXISTING", 2);

        assert_eq!(result, Err(TransferFundsError::DestinationUserNotFound));
        assert_eq!(bank.get_user_by_id("user2".to_string()).unwrap().balance, 1);
    }

    #[test]
    fn transfer_funds_amount_cannot_be_zero() {
        let user1 = User::new("user1".to_string(), 10, 1);
        let user2 = User::new("user2".to_string(), 1, 90);
        let mut bank = Bank::new(vec![user1, user2], "First Bank".to_string(), 1, 4);

        let result = bank.transfer_funds("user1", "user2", 0u64);

        assert_eq!(result, Err(TransferFundsError::InvalidAmount));
        assert_eq!(bank.get_user_by_id("user1".to_string()).unwrap().balance, 1);
        assert_eq!(
            bank.get_user_by_id("user2".to_string()).unwrap().balance,
            90
        );
    }

    #[test]
    fn bank_accrue_interest_for_debits() {
        let user1 = User::new("user1".to_string(), 0, 100);
        let mut bank = Bank::new(vec![user1], "First Bank".to_string(), 100, 400);

        let result = bank.accrue_interest();

        assert!(result);
        assert_eq!(
            bank.get_user_by_id("user1".to_string()).unwrap().balance,
            104
        );
    }

    #[test]
    fn bank_accrue_interest_for_credits() {
        let user1 = User::new("user1".to_string(), 100, -100);
        let mut bank = Bank::new(vec![user1], "First Bank".to_string(), 100, 400);

        let result = bank.accrue_interest();

        assert!(result);
        assert_eq!(
            bank.get_user_by_id("user1".to_string()).unwrap().balance,
            -101
        );
    }

    #[test]
    fn merge_banks_when_no_users_overlap() {
        let user1_bank1 = User::new("user1".to_string(), 1, 1);
        let user2_bank1 = User::new("user2".to_string(), 1, 2);
        let mut bank1 = Bank::new(
            vec![user1_bank1, user2_bank1],
            "First Bank".to_string(),
            1,
            4,
        );

        let user3_bank2 = User::new("user3".to_string(), 1, 3);
        let user4_bank2 = User::new("user4".to_string(), 1, 4);
        let bank2 = Bank::new(
            vec![user3_bank2, user4_bank2],
            "Second Bank".to_string(),
            1,
            4,
        );

        let result = bank1.merge(bank2);

        assert!(result);
        assert_eq!(
            bank1.get_user_by_id("user1".to_string()).unwrap().balance,
            1
        );
        assert_eq!(
            bank1.get_user_by_id("user2".to_string()).unwrap().balance,
            2
        );
        assert_eq!(
            bank1.get_user_by_id("user3".to_string()).unwrap().balance,
            3
        );
        assert_eq!(
            bank1.get_user_by_id("user4".to_string()).unwrap().balance,
            4
        );
    }

    #[test]
    fn merge_banks_should_merge_balances_when_users_overlap() {
        let balance_at_first_bank = 1;
        let user1 = "user1";
        let user1_bank1 = User::new(user1.to_string(), 1, balance_at_first_bank);
        let user2_bank1 = User::new("user2".to_string(), 1, 2);
        let mut bank1 = Bank::new(
            vec![user1_bank1, user2_bank1],
            "First Bank".to_string(),
            1,
            4,
        );

        let balance_at_second_bank = 1;
        let user1_bank2 = User::new(user1.to_string(), 1, balance_at_second_bank);
        let user3_bank2 = User::new("user3".to_string(), 1, 3);
        let bank2 = Bank::new(
            vec![user1_bank2, user3_bank2],
            "Second Bank".to_string(),
            1,
            4,
        );

        let result = bank1.merge(bank2);

        assert!(result);
        assert_eq!(
            bank1.get_user_by_id(user1.to_string()).unwrap().balance,
            balance_at_first_bank + balance_at_second_bank
        );
        assert_eq!(
            bank1.get_user_by_id("user2".to_string()).unwrap().balance,
            2
        );
        assert_eq!(
            bank1.get_user_by_id("user3".to_string()).unwrap().balance,
            3
        );
    }
}
