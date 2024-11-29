use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub enum NexbotRecorderInstruction {
    RecordTrade {
        trade_id: String,
        to_public_key: String,
        amount: u64,
    },
    RecordState {
        state: String,
        mood: String,
    },
}
