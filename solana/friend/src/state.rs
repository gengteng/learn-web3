pub mod user {
    use crate::utils::get_timestamp;
    use borsh::{BorshDeserialize, BorshSerialize};
    use solana_program::program_error::ProgramError;
    use solana_program::pubkey::Pubkey;

    pub const MAX_FOLLOWERS: usize = 200;

    pub trait Space {
        fn space() -> usize;
    }

    #[derive(Debug, Default, BorshDeserialize, BorshSerialize)]
    pub struct Profile {
        pub data_len: u16,
        pub follows: Vec<Pubkey>,
    }

    impl Space for Profile {
        fn space() -> usize {
            Self::calculate_space(MAX_FOLLOWERS)
        }
    }

    impl Profile {
        pub fn calculate_space(count: usize) -> usize {
            6 + count * size_of::<Pubkey>()
        }

        pub fn follow(&mut self, user: Pubkey) {
            self.follows.push(user);
            self.data_len += 1;
        }
    }

    #[derive(Debug, Default, BorshDeserialize, BorshSerialize)]
    pub struct PostStats {
        pub post_count: u64,
    }

    impl Space for PostStats {
        fn space() -> usize {
            8
        }
    }

    #[derive(Debug, BorshDeserialize, BorshSerialize)]
    pub struct Post {
        pub content: String,
        pub timestamp: u64,
    }

    impl Post {
        pub fn new(content: String) -> Result<Self, ProgramError> {
            Ok(Self {
                content,
                timestamp: get_timestamp()?,
            })
        }
    }

    impl Space for Post {
        fn space() -> usize {
            8 + 8 + 256
        }
    }
}
