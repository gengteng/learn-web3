import * as anchor from "@coral-xyz/anchor"
import { program } from "./wallet";
import { getNftAddress } from "./nft";

export async function nftStake(
    wallet: anchor.Wallet,
    nftId: string,
) {
    const nftAddress = getNftAddress(nftId);
    return await program.methods.nftStake()
        .accounts({
            signer: wallet.publicKey,
            nftMintAccount: nftAddress
        })
        .signers([wallet.payer])
        .rpc();
}

export async function nftWithdraw(
    wallet: anchor.Wallet,
    nftId: string,
) {
    const nftAddress = getNftAddress(nftId);
    return await program.methods.nftWithdraw()
        .accounts({
            signer: wallet.publicKey,
            nftMintAccount: nftAddress
        })
        .signers([wallet.payer])
        .rpc();
}