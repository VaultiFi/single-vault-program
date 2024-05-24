import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { SingleVaultProgram } from "../target/types/single_vault_program";
import { Keypair } from "@solana/web3.js";

describe("single-vault-program", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.SingleVaultProgram as Program<SingleVaultProgram>;

  const vaultAccount = new Keypair();

  it("Is initialized!", async () => {
    // Add your test here.
    const tx = await program.methods
    .initialize()
    .signers([vaultAccount]) 
    .accounts({
      vaultAccount: vaultAccount.publicKey, // Pass the vault account to the program
    })
    .rpc({skipPreflight: true});
    
    console.log("Your transaction signature", tx);
  });
});
