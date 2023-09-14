use near_sdk::{
    env, near_bindgen, AccountId, Promise, ext_contract,
    collections::{PersistentMap, PersistentDeque},
    json_types::U128,
    near_sdk::borsh::{self, BorshDeserialize, BorshSerialize},
};

/**************************/
/* DATA TYPES AND VARIABLES */
/**************************/
type TokenId = u16;

/**************************/
/* STORAGE & COLLECTIONS */
/**************************/

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
struct FrameData {
    image_data: String,
    message: String,
    coauthor: String,
}

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
struct FrameMetadata {
    owner: AccountId,
    price: u32, // *possible feature* Pricing Mechanism should be added here for frames.
    created_at: u64,
}

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
struct HistoryEntry {
    frame_id: TokenId,
    buyer: AccountId,
    seller: AccountId,
    price: u32,
}

#[near_bindgen]
struct PixelArtContract {
    frame_data_map: PersistentMap<TokenId, FrameData>,
    frame_metadata_map: PersistentMap<TokenId, FrameMetadata>,
    history: PersistentDeque<HistoryEntry>,
    contract_owner: AccountId, // *possible feature* Access Control feature should track the contract owner here.
}

impl Default for PixelArtContract {
    fn default() -> Self {
        Self {
            frame_data_map: PersistentMap::new(b"f".to_vec()),
            frame_metadata_map: PersistentMap::new(b"m".to_vec()),
            history: PersistentDeque::new(b"h".to_vec()),
            contract_owner: env::current_account_id(), // *possible feature* Access Control feature should set the initial contract owner here.
        }
    }
}

/**************************/
/* READ METHODS */
/**************************/

#[near_bindgen]
impl PixelArtContract {
    pub fn load_frame_data(&self, start: u16, end: u16) -> Vec<String> {
        let mut output = Vec::new();

        for index in start..=end {
            output.push(self.frame_data_map.get(&index).unwrap());
        }

        output
    }

    pub fn load_frames(&self, start: u16, end: u16) -> FrameDataWrapper {
        let mut metadata_output = Vec::new();
        let mut data_output = Vec::new();

        for index in start..end {
            if let Some(frame_metadata) = self.frame_metadata_map.get(&index) {
                metadata_output.push(frame_metadata);
            }
            if let Some(frame_data) = self.frame_data_map.get(&index) {
                data_output.push(frame_data);
            }
        }

        FrameDataWrapper {
            metadata: metadata_output,
            data: data_output,
        }
    }

    // ... (other read methods)
}

/**************************/
/* WRITE METHODS */
/**************************/

#[near_bindgen]
impl PixelArtContract {
    // ... (implementation of write methods)
}

/**************************/
/* VIEW METHODS */
/**************************/

#[near_bindgen]
impl PixelArtContract {
    pub fn get_history(&self) -> Vec<HistoryEntry> {
        let mut history_response = Vec::new();
        let num_entries = std::cmp::min(self.history.len(), 20);

        for i in 0..num_entries {
            history_response.push(self.history.get(i).unwrap());
        }

        history_response
    }

    // ... (other view methods)
}

/**************************/
/* PRIVATE METHODS */
/**************************/

// ... (implementation of private methods)

/**************************/
/* EXTERNAL CONTRACTS */
/**************************/

// ... (implementation of external contracts)

/**************************/
/* UTILS */
/**************************/

// ... (implementation of utility functions)

/**************************/
/* ASSEMBLYSCRIPT EXPORTS */
/**************************/

#[ext_contract(ext_self)]
trait ExternalPixelArtContract {
    // ... (external contract methods)
}

#[derive(BorshDeserialize, BorshSerialize)]
pub struct FrameDataWrapper {
    pub metadata: Vec<FrameMetadata>,
    pub data: Vec<String>,
}

/**************************/
/* NEAR BINDING */
/**************************/

#[near_bindgen]
impl PixelArtContract {
    // ... (near binding methods)
}

/**************************/
/* UNIT TESTS */
/**************************/

// ... (unit tests)