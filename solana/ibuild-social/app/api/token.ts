import * as anchor from "@coral-xyz/anchor"
import { program } from "./wallet";

export async function createTokenMintAccount(
    wallet: anchor.Wallet,
): Promise<[anchor.web3.PublicKey, anchor.web3.TransactionSignature]> {
    const [splTokenMintAddress, _] = anchor.web3.PublicKey.findProgramAddressSync(
        [Buffer.from("mint")],
        program.programId
    );

    return [
        splTokenMintAddress, 
        await program.methods.createTokenMintAccount()
            .accounts({
                payer: wallet.publicKey
            })
            .signers([wallet.payer])
            .rpc()
    ];
}