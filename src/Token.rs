use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::{env, near_bindgen, AccountId, Balance, Promise};

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
pub struct Token {
    total_supply: Balance,
    balances: std::collections::HashMap<AccountId, Balance>,
    owner: AccountId,
}

impl Default for Token {
    fn default() -> Self {
        panic!("Token should be initialized before usage")
    }
}

#[near_bindgen]
impl Token {
    #[init]
    pub fn new(total_supply: Balance, owner_id: AccountId) -> Self {
        let mut ft = Self { total_supply, balances: std::collections::HashMap::new(), owner: owner_id };
        ft.balances.insert(owner_id, total_supply);
        ft
    }

    pub fn total_supply(&self) -> Balance {
        self.total_supply
    }

    pub fn balance_of(&self, account_id: AccountId) -> Balance {
        self.balances.get(&account_id).unwrap_or(&0).clone()
    }

    pub fn transfer(&mut self, receiver_id: AccountId, amount: Balance) {
        let sender_id = env::predecessor_account_id();
        let sender_balance = self.balances.get(&sender_id).unwrap_or(&0).clone();
        assert!(amount <= sender_balance, "Not enough balance");
        self.balances.insert(sender_id, sender_balance - amount);
        let receiver_balance = self.balances.get(&receiver_id).unwrap_or(&0).clone();
        self.balances.insert(receiver_id, receiver_balance + amount);
    }
}
