use anchor_lang::prelude::*;

declare_id!("VPgJKrxm2PNrWcF7mnspgqnRLboaoBdVkLcrPZ9qig8");

#[program]
pub mod single_vault_program {
    use super::*;

    pub fn initialize(_ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}