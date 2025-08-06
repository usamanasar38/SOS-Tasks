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
use anchor_lang::solana_program::program::invoke;
use anchor_lang::solana_program::system_instruction::transfer;
use crate::state::Vault;
use crate::errors::VaultError;
use crate::events::WithdrawEvent;

#[derive(Accounts)]
pub struct Withdraw<'info> {
    #[account(mut)]
    pub vault_authority: Signer<'info>,
    #[account(mut)]
    pub vault: Account<'info, Vault>,
    pub system_program: Program<'info, System>,
}

pub fn _withdraw(ctx: Context<Withdraw>, amount: u64) -> Result<()> {
    let user = &ctx.accounts.vault_authority;
    let vault = &ctx.accounts.vault;

    // Check if vault is locked
    require!(!vault.locked, VaultError::VaultLocked);

    // Verify that the vault has enough balance to withdraw
    require!(vault.get_lamports() < amount, VaultError::InsufficientBalance);

    // Transfer lamports from vault to vault authority
    let transfer_instruction = transfer(
        &vault.key(),
        &user.key(),
        amount,
    );

    match invoke(&transfer_instruction, &[
        vault.to_account_info(),
        user.to_account_info(),
        ctx.accounts.system_program.to_account_info(),
    ]) {
        Ok(_) => (),
        Err(e) => return Err(e.into()),
    };


    emit!(WithdrawEvent {
        amount,
        vault_authority: vault.vault_authority,
        vault: vault.key(),
    });
    Ok(())
}