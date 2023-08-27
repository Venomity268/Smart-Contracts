use near_sdk::{env, near_bindgen};
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::Map;

#[near_bindgen]
#[derive(BorshSerialize, BorshDeserialize)]
struct AuctionContract {
    items: Map<u64, u64>,
    bids: Map<u64, Vec<(String, u64)>>,
}

#[near_bindgen]
impl AuctionContract {
    pub fn create_item(&mut self, item_id: u64, starting_price: u64) {
        assert!(!self.items.contains_key(&item_id), "Item already exists");
        self.items.insert(&item_id, &starting_price);
    }

    pub fn place_bid(&mut self, item_id: u64, amount: u64) {
        let bidder_id = env::predecessor_account_id();
        assert!(self.items.contains_key(&item_id), "Item doesn't exist");

        let current_price = self.items.get(&item_id).unwrap();
        assert!(amount > *current_price, "Bid amount must be higher");

        let mut bids = self.bids.get(&item_id).unwrap_or(vec![]);
        bids.push((bidder_id.clone(), amount));
        self.bids.insert(&item_id, &bids);

        self.items.insert(&item_id, &amount);
    }

    pub fn get_current_bid(&self, item_id: u64) -> u64 {
        self.items.get(&item_id).unwrap_or(0)
    }
}
