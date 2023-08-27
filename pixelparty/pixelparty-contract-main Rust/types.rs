use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::json_types::{U128, Base58PublicKey};

#[derive(BorshDeserialize, BorshSerialize)]
pub struct FrameMetadata {
    pub price: u32,
    pub owner: Base58PublicKey,
    pub message: String,
    pub coauthor: String,
    pub ts: u64,
}

#[derive(BorshDeserialize, BorshSerialize)]
pub struct HistoryEntry {
    pub frame_id: u16,
    pub buyer: Base58PublicKey,
    pub seller: Base58PublicKey,
    pub price: U128,
}

#[derive(BorshDeserialize, BorshSerialize)]
pub struct AddEggMessage {
    pub account_id: String,
    pub rarity: u32,
}

#[derive(BorshDeserialize, BorshSerialize)]
pub struct FrameDataWrapper {
    pub metadata: Vec<FrameMetadata>,
    pub data: Vec<String>,
}