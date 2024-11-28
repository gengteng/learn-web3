use friend::instruction::{SeedType, SocialInstruction};
use friend::solana_program::pubkey::Pubkey;
use solana_client::rpc_client::RpcClient;
use solana_sdk::instruction::{AccountMeta, Instruction};
use solana_sdk::signature::{Keypair, Signature, Signer};

pub struct Client {
    rpc_client: RpcClient,
    program_id: Pubkey,
}

impl Client {
    pub fn new(url: impl Into<String>, program_id: Pubkey) -> Self {
        Self {
            rpc_client: RpcClient::new(url.into()),
            program_id,
        }
    }

    pub fn initialize_user(
        &self,
        user: &Keypair,
        seed_type: SeedType,
    ) -> anyhow::Result<Signature> {
        let pda = get_pda_address(
            self.program_id,
            &[user.pubkey().as_ref(), seed_type.to_str().as_bytes()],
        );

        let initialize_user_ix = Instruction::new_with_borsh(
            self.program_id,
            &SocialInstruction::InitializeUser { seed_type },
            vec![
                AccountMeta::new(user.pubkey(), true),
                AccountMeta::new(pda, false),
                AccountMeta::new_readonly(solana_sdk::system_program::id(), false),
            ],
        );

        let signature = self.send_instruction(user, &[initialize_user_ix])?;

        Ok(signature)
    }

    pub fn follow(&self, user: &Keypair, followed: Pubkey) -> anyhow::Result<Signature> {
        let pda = get_pda_address(
            self.program_id,
            &[
                user.pubkey().as_ref(),
                SeedType::Profile.to_str().as_bytes(),
            ],
        );
        let follow_ix = Instruction::new_with_borsh(
            self.program_id,
            &SocialInstruction::FollowUser { user: followed },
            vec![AccountMeta::new(pda, false)],
        );

        let signature = self.send_instruction(user, &[follow_ix])?;

        Ok(signature)
    }

    pub fn query_followers(&self, user: &Keypair) -> anyhow::Result<Signature> {
        let pda = get_pda_address(
            self.program_id,
            &[
                user.pubkey().as_ref(),
                SeedType::Profile.to_str().as_bytes(),
            ],
        );
        let query_followers_ix = Instruction::new_with_borsh(
            self.program_id,
            &SocialInstruction::QueryFollowers,
            vec![AccountMeta::new(pda, false)],
        );

        let signature = self.send_instruction(user, &[query_followers_ix])?;

        Ok(signature)
    }

    fn send_instruction(
        &self,
        payer: &Keypair,
        instructions: &[Instruction],
    ) -> anyhow::Result<Signature> {
        let last_block_hash = self.rpc_client.get_latest_blockhash()?;
        let transaction = solana_sdk::transaction::Transaction::new_signed_with_payer(
            instructions,
            Some(&payer.pubkey()),
            &[payer],
            last_block_hash,
        );
        Ok(self.rpc_client.send_and_confirm_transaction(&transaction)?)
    }
}

fn get_pda_address(program_id: Pubkey, seed: &[&[u8]]) -> Pubkey {
    Pubkey::find_program_address(seed, &program_id).0
}
