use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct Post {
    #[max_len(64)]
    pub content: String,
    pub author: Pubkey,
    pub like_count: u64,
}

impl Post {
    pub const SEED_PREFIX: &'static [u8] = b"post";
}
