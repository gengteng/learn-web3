import { createLike, getLike } from "./api/like";
import { getNftAddress, mintNft } from "./api/nft";
import { createPost, getPost } from "./api/post";
import { createProfile, getProfile } from "./api/profile";
import { nftStake, nftWithdraw } from "./api/stake";
import { createTokenMintAccount } from "./api/token";
import { getDefaultWallet, getVisitorWallet } from "./api/wallet"

(async () => {
    const defaultWallet = getDefaultWallet();
    const visitorWallet = getVisitorWallet();

    // const txr0 = await createProfile(defaultWallet, "Alice");
    // console.log(txr0);

    // const result0 = await getProfile(defaultWallet);
    // console.log(result0);

    // const txr1 = await createProfile(visitorWallet, "Bob");
    // console.log(txr1);

    // const result1 = await getProfile(visitorWallet);
    // console.log(result1);

    // const [postAddress, txr2] = await createPost(defaultWallet, "Hello, World!");
    // console.log(txr2);

    // const result2 = await getPost(defaultWallet, postAddress);
    // console.log(result2);

    // const liketx = await createLike(visitorWallet, postAddress);
    // console.log(liketx);

    // const result3 = await getPost(defaultWallet, postAddress);
    // console.log(result3);

    const [pda, createMintTxr] = await createTokenMintAccount(defaultWallet);
    console.log("Token mint account: " + pda);
    console.log(createMintTxr);

    // const mintNftTxr = await mintNft(defaultWallet, "ONE");
    // console.log(mintNftTxr);

    const addr = await getNftAddress("ONE");
    console.log(addr.toBase58());

    // const txr = await nftStake(defaultWallet, "ONE");
    // console.log(txr);

    const txr = await nftWithdraw(defaultWallet, "ONE");
    console.log(txr);
})()