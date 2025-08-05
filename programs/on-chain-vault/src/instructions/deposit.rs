//-------------------------------------------------------------------------------
///
/// TASK: Implement the deposit functionality for the on-chain vault
/// 
/// Requirements:
/// - Verify that the user has enough balance to deposit
/// - Verify that the vault is not locked
/// - Transfer lamports from user to vault using CPI (Cross-Program Invocation)
/// - Emit a deposit event after successful transfer
/// 
///-------------------------------------------------------------------------------

use anchor_lang::prelude::*;
use anchor_lang::solana_program::program::invoke;
use anchor_lang::solana_program::system_instruction::transfer;
use crate::state::Vault;
use crate::errors::VaultError;
use crate::events::DepositEvent;

#[derive(Accounts)]
pub struct Deposit<'info> {
    #[account(mut)]
    pub user: Signer<'info>,
    #[account(
        init, 
        payer = user, 
        // space = discriminant + account size
        space = 8 + Vault::INIT_SPACE,
        seeds = [b"vault", user.key().as_ref()],
        bump
    )]
    pub vault: Account<'info, Vault>,
    pub system_program: Program<'info, System>,
}

pub fn _deposit(ctx: Context<Deposit>, amount: u64) -> Result<()> {
    let user = &ctx.accounts.user;
    let vault = &ctx.accounts.vault;

    // Check if the user has enough balance to deposit
    require!(user.lamports() >= amount, VaultError::VaultLocked);
    
    // Check if vault is locked
    require!(!vault.locked, VaultError::VaultLocked);

    // Transfer lamports from user to vault
    let transfer_instruction = transfer(
        &user.key(),
        &vault.key(),
        amount,
    );

    invoke(&transfer_instruction, &[
        user.to_account_info(),
        vault.to_account_info(),
        ctx.accounts.system_program.to_account_info(),
    ])?;


    emit!(DepositEvent {
        user: ctx.accounts.user.key(),
        vault: ctx.accounts.vault.key(),
        amount,
    });
    Ok(())
}