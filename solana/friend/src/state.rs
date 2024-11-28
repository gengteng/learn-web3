pub mod user {
    use crate::utils::get_timestamp;
    use borsh::{BorshDeserialize, BorshSerialize};
    use solana_program::program_error::ProgramError;
    use solana_program::pubkey::Pubkey;
    use std::mem::size_of;

    pub const MAX_FOLLOWERS: usize = 200;

    pub const VEC_LENGTH_LEN: usize = 4;

    pub trait Space {
        fn space() -> usize;
    }

    #[derive(Debug, Default, BorshDeserialize, BorshSerialize)]
    pub struct Profile {
        pub follows: Vec<Pubkey>,
    }

    impl Space for Profile {
        fn space() -> usize {
            Self::calculate_space(MAX_FOLLOWERS)
        }
    }

    impl Profile {
        pub fn calculate_space(count: usize) -> usize {
            VEC_LENGTH_LEN + count * size_of::<Pubkey>()
        }

        pub fn follow(&mut self, user: Pubkey) {
            self.follows.push(user);
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

#[cfg(test)]
mod tests {
    use crate::state::user::Profile;
    use solana_program::pubkey::Pubkey;

    #[test]
    fn test_profile_space() {
        let mut profile = Profile::default();
        println!("{}", borsh::to_vec(&profile).unwrap().len());
        profile.follow(Pubkey::new_unique());
        println!("{}", borsh::to_vec(&profile).unwrap().len());
        profile.follow(Pubkey::new_unique());
        println!("{}", borsh::to_vec(&profile).unwrap().len());
    }
}
