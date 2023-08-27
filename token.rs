use near_sdk::{env, near_bindgen};
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::UnorderedMap;

#[near_bindgen]
#[derive(BorshSerialize, BorshDeserialize)]
struct TokenContract {
    balances: UnorderedMap<String, u64>,
}

#[near_bindgen]
impl TokenContract {
    pub fn transfer(&mut self, receiver_id: String, amount: u64) {
        let sender_id = env::predecessor_account_id();
        let sender_balance = self.balances.get(&sender_id).unwrap_or(0);
        assert!(sender_balance >= amount, "Not enough balance");

        self.balances.insert(&sender_id, &(sender_balance - amount));
        self.balances.insert(&receiver_id, &(self.balances.get(&receiver_id).unwrap_or(0) + amount));
    }

    pub fn get_balance(&self, account_id: String) -> u64 {
        self.balances.get(&account_id).unwrap_or(0)
    }
}
