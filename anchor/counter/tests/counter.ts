import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Counter } from "../target/types/counter";

describe("counter", () => {
  // Configure the client to use the local cluster.
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const program = anchor.workspace.Counter as Program<Counter>;

  const wallet = provider.wallet as anchor.Wallet;

  // const keypair = anchor.web3.Keypair.generate();
  // console.log("Keypair", keypair.publicKey.toBase58());

  // it("Is initialized!", async () => {
  //   // Add your test here.
  //   const tx = await program.methods.initialize().accounts({
  //     payer: wallet.publicKey,
  //     counter: keypair.publicKey,
  //   }).signers([wallet.payer, keypair]).rpc();
  //   console.log("Your transaction signature", tx);
  // });

  it("Increment!", async () => {
    const pubkey = new anchor.web3.PublicKey("2FnCzKK3c696L1qT3PfQnJAA5zs2mAhGHwNEj6BPUpQP");
    // Add your test here.
    const tx = await program.methods.increment().accounts({
      counter: pubkey,
    }).rpc();
    console.log("Your transaction signature", tx);
  });

  it("query counter", async () => {
    const pubkey = new anchor.web3.PublicKey("2FnCzKK3c696L1qT3PfQnJAA5zs2mAhGHwNEj6BPUpQP");
    const counter = await program.account.counter.fetch(pubkey);
    console.log("Counter", counter);
  });
});
