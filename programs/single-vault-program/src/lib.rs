use anchor_lang::prelude::*;

declare_id!("5z6TMA2KJS99AJtZSn51mGLj394Sq4dfcoh36ZmVKeuk");

#[program]
// Instructions being executed by the program
pub mod single_vault_program { 
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        let vault_account = &mut ctx.accounts.vault_account;    // Initialize the vault_account
        vault_account.bump = ctx.bumps.vault_account;                                       // Store the bump seed in the vault_account account.
        msg!("VaultAccount is Initialized.");                                               // Returns a message on the Blockchain Logs that the VaultAccount is Initialized
        Ok(()) 
    }
}

// Initialize program instructions
#[derive(Accounts)] 
pub struct Initialize<'info> {
    #[account(mut)]
    pub user: Signer<'info>, 

    #[account(
        init,                      // Initialize the vault_account
        seeds = [b"testVault1"],   // This is the seed for the vault_account
        bump,                      // This is the bump seed for the vault_account (which is a PDA) 
        payer = user,              // The user is the payer for the vault_account
        space = 256,               // 256 bytes of space for the vault_account
    )]
    // Initialize accounts and the system_program (native program)
    pub vault_account: Account<'info, VaultAccount>,
    pub system_program: Program<'info, System>,      // System Program Initializing the vault_account
}

// This is a Validation Struct in which allows the vault_account PDA to used again in a different instruction. The bump seed for that account is referrenced by bump = vault_account.bump
#[derive(Accounts)]
pub struct VaultInformation<'info> { 
    #[account(
        mut,
        seeds = [b"testVault1"],                     // optional seeds for the vault_account i.e. "seeds + Keypair"
        bump = vault_account.bump,                   // bump seed used for the vault_account PDA. Initially the bump + seed was created in the Initialize instruction struct.
    )]
    pub vault_account: Account<'info, VaultAccount>, // Let the instruction know which account to interact with.
}

// All accounts used and the Data that those accounts require in order fo the instructions to be executed.
#[account] 
// Struct for the Vault Account Data that will be used in the program 
pub struct VaultAccount { 
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
    bump: u8, 
}