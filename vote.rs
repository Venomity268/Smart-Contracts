use near_sdk::{env, near_bindgen};
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::Map;

#[near_bindgen]
#[derive(BorshSerialize, BorshDeserialize)]
struct VotingContract {
    options: Map<String, u64>,
    votes: Map<String, u64>,
}

#[near_bindgen]
impl VotingContract {
    pub fn add_option(&mut self, option: String) {
        assert!(!self.options.contains_key(&option), "Option already exists");
        self.options.insert(&option, &0);
    }

    pub fn vote(&mut self, option: String) {
        let voter_id = env::predecessor_account_id();
        assert!(self.options.contains_key(&option), "Option doesn't exist");

        let vote_count = self.votes.get(&voter_id).unwrap_or(0);
        self.votes.insert(&voter_id, &(vote_count + 1));

        let option_count = self.options.get(&option).unwrap_or(0);
        self.options.insert(&option, &(option_count + 1));
    }

    pub fn get_votes(&self, option: String) -> u64 {
        self.options.get(&option).unwrap_or(0)
    }
}
