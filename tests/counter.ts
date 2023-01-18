import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import { Counter } from "../target/types/counter";

describe("counter", () => {
  // Configure the client to use the local cluster.
  const counter = anchor.web3.Keypair.generate()

  const provider = anchor.AnchorProvider.env()
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.Counter as Program<Counter>;

  it("Is initialized!", async () => {
    // Add your test here.
    const tx = await program.methods.initialize().accounts({
      counter:counter.publicKey ,
      payer:provider.wallet.publicKey,
      systemProgram:anchor.web3.SystemProgram.programId
    }).signers([counter]).rpc();

    const counterData = await program.account.counter.fetch(counter.publicKey);
    console.log(counterData)
    console.log("Your transaction signature", tx);
  });

  it("Updates correctly", async() => {
  

    await program.methods.update(5).accounts({
      counter:counter.publicKey,
      payer:provider.wallet.publicKey,
    }).rpc();
    const counterData = await program.account.counter.fetch(counter.publicKey);
    console.log(counterData)
  
  })

});
