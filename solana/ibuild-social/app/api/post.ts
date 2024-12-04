import * as anchor from '@coral-xyz/anchor';
import { program } from './wallet';

export async function createPost(
    wallet: anchor.Wallet, 
    content: string

): Promise<[anchor.web3.PublicKey, anchor.web3.TransactionSignature]> {
    // find profile pda
    const [profileAddress,] = anchor.web3.PublicKey.findProgramAddressSync([Buffer.from("profile"), wallet.publicKey.toBuffer()], program.programId);
    console.log("Profile Address: " + profileAddress);

    const profile = await program.account.iBuidlProfile.fetch(profileAddress);
    const index = profile.postCount.add(new anchor.BN(1));
    // convert index to le bytes
    const indexBuffer = Buffer.alloc(8);
    index.toArray().forEach((b, i) => indexBuffer.writeUInt8(b, i));

    const [postAddress,] = anchor.web3.PublicKey.findProgramAddressSync([Buffer.from("post"), wallet.publicKey.toBuffer(), indexBuffer], program.programId);
    console.log("Post Address: " + postAddress);

    return [postAddress, await program.methods.createPost(content)
        .accounts({
            signer: wallet.publicKey,
            post: postAddress,
        })
        .signers([wallet.payer])
        .rpc()];
}

export async function getPost(wallet: anchor.Wallet, postAddress: anchor.web3.PublicKey): Promise<any> {
    return await program.account.post.fetch(postAddress);
}