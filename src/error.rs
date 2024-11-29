use thiserror::Error;
use solana_program::program_error::ProgramError;

#[derive(Error, Debug, Copy, Clone)]
pub enum NexbotRecorderError {
    #[error("Invalid instruction")]
    InvalidInstruction,
}

impl From<NexbotRecorderError> for ProgramError {
    fn from(e: NexbotRecorderError) -> Self {
        ProgramError::Custom(e as u32)
    }
}
