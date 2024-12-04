use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct Like {
    pub post: Pubkey,
    pub profile: Pubkey,
}

impl Like {
    pub const SEED_PREFIX: &'static [u8] = b"like";

    pub fn update(&mut self, post: Pubkey, profile: Pubkey) {
        self.post = post;
        self.profile = profile;
    }
}
