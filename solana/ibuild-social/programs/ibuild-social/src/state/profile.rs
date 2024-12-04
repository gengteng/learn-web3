use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct IBuidlProfile {
    #[max_len(32)]
    pub display_name: String,
    pub post_count: u64,
}

impl IBuidlProfile {
    pub const SEED_PREFIX: &'static [u8] = b"profile";
}
