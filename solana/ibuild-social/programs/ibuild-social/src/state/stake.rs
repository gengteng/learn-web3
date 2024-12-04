use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct StakeInfo {
    pub staker: Pubkey,
    pub nft_mint: Pubkey,
    pub stake_at: u64,
}

impl StakeInfo {
    pub const SEED_PREFIX: &'static [u8] = b"stake";

    pub fn new(staker: Pubkey, nft_mint: Pubkey, stake_at: u64) -> Self {
        Self {
            staker,
            nft_mint,
            stake_at,
        }
    }

    pub fn salvage_value(&self, current_epoch: u64, amount: u64) -> u64 {
        msg!("current_epoch: {}", current_epoch);
        msg!("stake_at: {}", self.stake_at);
        let elapsed_epochs = current_epoch - self.stake_at;
        // 每个纪元减少 2%
        let deduct = (amount * elapsed_epochs * 2) / 100;
        msg!("deduct: {}", deduct);
        if amount > deduct {
            amount - deduct
        } else {
            0
        }
    }
}