use anchor_lang::prelude::*;

declare_id!("12i6Bo93TrMuSmk9cMiVT1yt1P2hcE8hkB92EWe6YnS5");

#[program]
// Instructions being executed by the program
pub mod single_vault_program { 
    use super::*;

    pub fn initialize(_ctx: Context<Initialize>) -> Result<()> {
        //let vault_account = &mut ctx.accounts.vault_account;    // Initialize the vault_account
        //vault_account.bump = ctx.bumps.vault_account;                                       // Store the bump see in the vault_account account.
        
        msg!("VaultAccount is Initialized.");    // Returns a message that the VaultAccount is Initialized
        Ok(()) 
    }
}

// Initialize program instructions
#[derive(Accounts)] 
pub struct Initialize<'info> {
    // The user account is mutable because the vault_account is being created by the user and the user is paying for the space.
    // The vault_account will interact with the user and modify the user account at a later time, hence its mutability.
    #[account(mut)]
    pub user: AccountInfo<'info>,

    #[account(
        init,                      // Initialize the vault_account
        seeds = [b"vaultaccount"], // This is the seed for the vault_account
        bump,                      // This is the bump seed for the vault_account (which is a PDA) 
        payer = user,              // The user is the payer for the vault_account
        space = 256,               // 256 bytes of space for the vault_account
    )]
    // Initialize accounts and the system_program (native program)
    pub vault_account: Account<'info, VaultAccount>,
    pub system_program: Program<'info, System>,     // System Program Initializing the vault_account
}

// Vault Information instructions on how to use the VaultAccount Data.
#[derive(Accounts)]
pub struct VaultInformation<'info> {
    #[account(
        mut,
        seeds = [b"vaultaccount"],                  // optional seeds for the vault_account i.e. "vaultaccount + Keypair"
        bump = vault_account.bump,                  // bump seed for PDA stored in the "vault_account" Account
    )]
    pub vault_account: Account<'info, VaultAccount>,// Let the instruction know which account to interact with.
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