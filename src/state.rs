use borsh::{BorshDeserialize, BorshSerialize};

#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub struct NexbotTrade {
    pub trade_id: String,
    pub to_public_key: String,
    pub amount: u64,
}

#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub struct NexbotState {
    pub state: String,
    pub mood: String,
}
