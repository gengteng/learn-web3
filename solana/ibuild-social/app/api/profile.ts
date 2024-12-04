import * as anchor from '@coral-xyz/anchor';
import { program } from './wallet';

export async function createProfile(wallet: anchor.Wallet, displayName: string): Promise<anchor.web3.TransactionSignature> {
    return await program.methods.createProfile(displayName)
        .accounts({
            payer: wallet.publicKey
        })
        .signers([wallet.payer])
        .rpc();
}

export async function getProfile(wallet: anchor.Wallet): Promise<any> {
    const [address, _] = anchor.web3.PublicKey.findProgramAddressSync([Buffer.from("profile"), wallet.publicKey.toBuffer()], program.programId);
    return await program.account.iBuidlProfile.fetch(address);
}