use crate::instruction::NexbotRecorderInstruction;
use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{
    account_info::AccountInfo, entrypoint::ProgramResult, msg, pubkey::Pubkey,
};

pub fn process_instruction(
    _program_id: &Pubkey,
    _accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    let instruction = NexbotRecorderInstruction::try_from_slice(instruction_data)?;
    match instruction {
        NexbotRecorderInstruction::RecordTrade {
            trade_id,
            to_public_key,
            amount,
        } => {
            msg!("Recording trade: {}, {}, {}", trade_id, to_public_key, amount);
            // Additional on-chain logic can go here
        }
        NexbotRecorderInstruction::RecordState { state, mood } => {
            msg!("Recording state: {}, {}", state, mood);
            // Additional on-chain logic can go here
        }
    }
    Ok(())
}
