use anchor_lang::prelude::*;

declare_id!("VPgJKrxm2PNrWcF7mnspgqnRLboaoBdVkLcrPZ9qig8");

#[program]
pub mod single_vault_program { // Instructions being executed by the program
    use super::*;

    pub fn initialize(_ctx: Context<Initialize>) -> Result<()> {
        
        Ok(())
    }
}

#[derive(Accounts)] // Instructions for the accounts
pub struct Initialize<'info> {
    #[account(mut)]
    pub user: AccountInfo<'info>,

    #[account(
        init,
        payer = user,
        space = 1024,
    )]

    pub vault_account: Account<'info, VaultAccount>,
    pub system_program: Program<'info, System>, // System Program Initializing the vault_account
}

#[account] // All accounts used and the Data that those accounts require in order fo the instructions to be executed

pub struct VaultAccount { // Struct for the Vault Account
    is_active: bool,
    vault_name: String,
    token_ticker_symbol: String,
    token_account: Pubkey,
    mint_address: Pubkey,
    reward_token: Pubkey,
    reward_amount: f64,
    rewards_remaining: f64,
    time: i64,
    total_tokens_vaulted: f64,
    total_rewards_earned: f64,
    expected_rewards: f64,
    desposit_amount: f64,
    annual_percentage_yield_fixed: f64,
    annual_percentage_yield_variable: f64,
}