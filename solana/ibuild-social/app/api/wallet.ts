import * as anchor from "@coral-xyz/anchor"
import { Program } from "@coral-xyz/anchor";
import { IbuildSocial } from "../../target/types/ibuild_social";

let provider = anchor.AnchorProvider.env();
anchor.setProvider(provider);

const program = anchor.workspace.IbuildSocial as Program<IbuildSocial>;

export { program, provider };

export function getDefaultWallet() {
    return anchor.Wallet.local();
}

// PubKey: FYGpKwM8JLPrjZpmyiRTqnSJNzibtXjMfA64aFEtLNpC
export function getVisitorWallet() {
    const keypair = anchor.web3.Keypair.fromSecretKey(
        new Uint8Array(
            [130,9,173,43,136,198,48,85,151,88,101,32,39,233,182,25,68,65,148,231,129,226,77,53,117,182,117,17,134,65,2,43,216,6,129,86,180,3,203,26,27,229,53,211,53,1,192,146,174,169,207,214,93,134,11,228,200,98,153,212,121,97,109,237]
        )
    );

    return new anchor.Wallet(keypair);
}