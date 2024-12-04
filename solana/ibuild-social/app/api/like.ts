import * as anchor from '@coral-xyz/anchor';
import { program } from './wallet';

export async function createLike(
    wallet: anchor.Wallet,
    postAddress: anchor.web3.PublicKey,
): Promise<anchor.web3.TransactionSignature> {
    const post = await program.account.post.fetch(postAddress);

    return await program.methods.createLike()
        .accounts({
            payer: wallet.publicKey,
            post: postAddress,
            authorWallet: post.author,
        })
        .signers([wallet.payer])
        .rpc();
}

export async function getLike(wallet: anchor.Wallet, likeAddress: anchor.web3.PublicKey): Promise<any> {
    return await program.account.like.fetch(likeAddress);
}