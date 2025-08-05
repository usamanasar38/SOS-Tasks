//-------------------------------------------------------------------------------
///
/// TASK: Implement the toggle lock functionality for the on-chain vault
/// 
/// Requirements:
/// - Toggle the locked state of the vault (locked becomes unlocked, unlocked becomes locked)
/// - Only the vault authority should be able to toggle the lock
/// - Emit a toggle lock event after successful state change
/// 
///-------------------------------------------------------------------------------

use anchor_lang::prelude::*;
use crate::state::Vault;
use crate::events::ToggleLockEvent;

#[derive(Accounts)]
pub struct ToggleLock<'info> {
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

pub fn _toggle_lock(ctx: Context<ToggleLock>) -> Result<()> {
    let vault = &mut ctx.accounts.vault;

    // Toggle the locked state
    vault.locked = !vault.locked;

    emit!(ToggleLockEvent {
        vault: vault.key(),
        vault_authority: vault.vault_authority,
        locked: vault.locked,
    });

    Ok(())
}