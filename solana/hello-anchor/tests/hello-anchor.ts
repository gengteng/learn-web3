import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { HelloAnchor } from "../target/types/hello_anchor";

describe("hello-anchor", () => {
  // Configure the client to use the local cluster.
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);
  console.log("Provider key: ", provider);

  let todo_list = anchor.web3.Keypair.generate();
  console.log("Todo List Key: ", todo_list.publicKey.toBase58());

  const program = anchor.workspace.HelloAnchor as Program<HelloAnchor>;

  const programId = program.programId;

  it("Create a todo list!", async () => {
    // Add your test here.
    const tx = await program.methods.createTodoList().accounts({
      signer: provider.publicKey,
      todoList: todo_list.publicKey,
    }).signers([todo_list]).rpc();
    console.log("Your transaction signature", tx);
  });

  it("Query the empty todo list", async () => {
    const todoList = await program.account.todoList.fetch(todo_list.publicKey);
    console.log("Todo List: ", todoList);
  });

  it("Add a todo item!", async () => {
    // Add your test here.
    const tx = await program.methods.addTodoItem("Hello, World!").accounts({
      signer: provider.publicKey,
      todoList: todo_list.publicKey,
    }).signers([]).rpc();
    console.log("Your transaction signature", tx);
  });

  it("Query the empty todo list", async () => {
    const todoList = await program.account.todoList.fetch(todo_list.publicKey);
    console.log("Todo List: ", todoList);
  });

  it("Remove a todo item!", async () => {
    // Add your test here.
    const tx = await program.methods.removeTodoItem("Hello, World!").accounts({
      signer: provider.publicKey,
      todoList: todo_list.publicKey,
    }).signers([]).rpc();
    console.log("Your transaction signature", tx);
  });

  it("Query the empty todo list", async () => {
    const todoList = await program.account.todoList.fetch(todo_list.publicKey);
    console.log("Todo List: ", todoList);
  });
});
