import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { SingleVaultProgram } from "/home/skippcode/solana_projects/vaultifi_organization/single-vault-program/target/types/single_vault_program";
import  { PublicKey } from "@solana/web3.js";

describe("single-vault-program", async () => {
  // Configure the client to use the local cluster.
  const provider = (anchor.AnchorProvider.env());

  anchor.setProvider(provider)

  const program = anchor.workspace.SingleVaultProgram as Program<SingleVaultProgram>;

  const [vaultAccountPDA] = PublicKey.findProgramAddressSync( // Find the Program Derived Address (PDA) for the vault account
    
  [Buffer.from("testVault1")], // Seed

    program.programId
  )

// "it" is a test case. This test case checks if the program is initialized.
  it("Is initialized!", async () => { 

    const tx = await program.methods
    .initialize()
    //.accounts({}) // Add accounts here
    .rpc({ skipPreflight: true})

    console.log("Transaction executed: ", tx);
  });
});
