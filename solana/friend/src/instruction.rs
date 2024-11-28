use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::pubkey::Pubkey;

#[derive(Debug, BorshSerialize, BorshDeserialize)]
pub enum SocialInstruction {
    InitializeUser { seed_type: SeedType },
    FollowUser { user: Pubkey },
    UnfollowUser { user: Pubkey },
    QueryFollowers,
    PostContent { content: String },
    QueryPosts,
}

#[derive(Debug, BorshSerialize, BorshDeserialize, Copy, Clone)]
pub enum SeedType {
    Profile,
    Post,
}

impl SeedType {
    pub fn to_str(&self) -> &str {
        match self {
            SeedType::Post => "post",
            SeedType::Profile => "profile",
        }
    }
}
