import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Calculator } from "../target/types/calculator";

describe("calculator", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.calculator as Program<Calculator>;

  it("Is initialized!", async () => {
    // Add your test here.
    const tx = await program.methods.initialize().rpc();
    console.log("Your transaction signature", tx);
  });


  it("Is addition!", async () => {
    // Add your test here.
    const tx = await program.methods.addition(new anchor.BN(2), new anchor.BN(4)).rpc();
    console.log("Your transaction signature", tx);
  });

   it("Is substraction!", async () => {
    // Add your test here.
    const tx = await program.methods.substraction(new anchor.BN(20), new anchor.BN(4)).rpc();
    console.log("Your transaction signature", tx);
  });

   it("Is multiplication!", async () => {
    // Add your test here.
    const tx = await program.methods.multiplication(new anchor.BN(2), new anchor.BN(4)).rpc();
    console.log("Your transaction signature", tx);
  });
  
});
