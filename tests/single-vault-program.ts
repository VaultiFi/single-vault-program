import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { SingleVaultProgram } from "../target/types/single_vault_program";
import { PublicKey } from "@solana/web3.js";

describe("single-vault-program", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.SingleVaultProgram as Program<SingleVaultProgram>;

  const [vaultaccountPDA] = PublicKey.findProgramAddressSync( // Find the Program Derived Address (PDA) for the vault account
    [Buffer.from("vaultaccount")],
    program.programId
  )

  it("Is initialized!", async () => {
    try {                                                     // Call the initialize method
      const tx = await program.methods
      .initialize()
      .rpc()
      console.log("Your transaction signature", tx);
    } catch (error) {                                         // Catch any errors
      console.log(error);
    }
    
  });
});
