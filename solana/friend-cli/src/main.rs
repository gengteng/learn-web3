mod client;

use friend::instruction::SeedType;
use solana_sdk::pubkey::Pubkey;
use solana_sdk::signature::{read_keypair_file, Keypair};
use std::str::FromStr;

fn main() -> anyhow::Result<()> {
    let program_id = Pubkey::from_str("2UCB6s1rBQGPVqyT2vAHPTEP3otVd37m279nmJKVXwvJ")?;
    let client = client::Client::new("http://localhost:8899", program_id);
    let user = read_keypair_file("/Users/gengteng/.config/solana/test.json")
        .map_err(|e| anyhow::anyhow!("{e}"))?;

    let signature = client.initialize_user(&user, SeedType::Profile)?;
    println!("Initialize Signature: {}", signature);

    let followed = Pubkey::new_unique();
    println!("Followed: {}", followed);

    let signature = client.follow(&user, followed)?;
    println!("Follow Signature: {}", signature);

    let signature = client.query_followers(&user)?;
    println!("Query Signature: {}", signature);
    Ok(())
}
