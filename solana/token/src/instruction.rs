use borsh::{BorshDeserialize, BorshSerialize};

#[derive(Debug, BorshDeserialize, BorshSerialize)]
pub enum TokenInstruction {
    CreateToken { decimal: u8 },
    Mint { amount: u64 },
}
