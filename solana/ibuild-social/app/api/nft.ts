import * as anchor from "@coral-xyz/anchor"
import { program } from "./wallet";

export async function mintNft(
    wallet: anchor.Wallet,
    nftId: string,
) {
    return await program.methods.mintNft(nftId)
        .accounts({
            payer: wallet.publicKey
        })
        .signers([wallet.payer])
        .rpc();
}

export function getNftAddress(id: string) {
    const [addr, ] =  anchor.web3.PublicKey.findProgramAddressSync(
        [Buffer.from("nft"), Buffer.from(id)],
        program.programId
    );
    return addr;
}