//-------------------------------------------------------------------------------
///
/// TASK: Implement the withdraw functionality for the on-chain vault
/// 
/// Requirements:
/// - Verify that the vault is not locked
/// - Verify that the vault has enough balance to withdraw
/// - Transfer lamports from vault to vault authority
/// - Emit a withdraw event after successful transfer
/// 
///-------------------------------------------------------------------------------

use anchor_lang::prelude::*;
use crate::state::Vault;
use crate::errors::VaultError;
use crate::events::WithdrawEvent;

#[derive(Accounts)]
pub struct Withdraw<'info> {
    #[account(mut)]
    pub vault_authority: Signer<'info>,
    #[account(
        init, 
        payer = vault_authority, 
        // space = discriminant + account size
        space = 8 + Vault::INIT_SPACE,
        seeds = [b"vault", vault_authority.key().as_ref()],
        bump
    )]
    pub vault: Account<'info, Vault>,
    pub system_program: Program<'info, System>,
}

pub fn _withdraw(ctx: Context<Withdraw>, amount: u64) -> Result<()> {
    let user = &ctx.accounts.vault_authority;
    let vault = &ctx.accounts.vault;

    // Check if vault is locked
    require!(!vault.locked, VaultError::VaultLocked);

    // Verify that the vault has enough balance to withdraw
    require!(vault.get_lamports() >= amount, VaultError::InsufficientBalance);

    // Transfer lamports from vault to vault authority
    let transfer_instruction = anchor_lang::solana_program::system_instruction::transfer(
        &vault.key(),
        &user.key(),
        amount,
    );
}