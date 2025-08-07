//-------------------------------------------------------------------------------
use crate::errors::VaultError;
use crate::events::WithdrawEvent;
use crate::state::Vault;
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

#[derive(Accounts)]
pub struct Withdraw<'info> {
    #[account(mut, signer)]
    pub vault_authority: Signer<'info>,
    #[account(
        mut,
        has_one = vault_authority, // Ensure the vault authority matches
        seeds = [b"vault", vault_authority.key().as_ref()],
        bump,
    )]
    pub vault: Account<'info, Vault>,
    pub system_program: Program<'info, System>,
}

pub fn _withdraw(ctx: Context<Withdraw>, amount: u64) -> Result<()> {
    // let user = &ctx.accounts.vault_authority;
    let vault = &ctx.accounts.vault;

    // Check if vault is locked
    require!(!vault.locked, VaultError::VaultLocked);

    // Verify that the vault has enough balance to withdraw
    let vault_balance = vault.to_account_info().lamports();
    require_gte!(vault_balance, amount, VaultError::InsufficientBalance);

    // Transfer lamports from vault to vault authority using invoke_signed
    **ctx
        .accounts
        .vault
        .to_account_info()
        .try_borrow_mut_lamports()? -= amount;
    **ctx
        .accounts
        .vault_authority
        .to_account_info()
        .try_borrow_mut_lamports()? += amount;
    emit!(WithdrawEvent {
        amount,
        vault_authority: vault.vault_authority,
        vault: vault.key(),
    });
    Ok(())
}
