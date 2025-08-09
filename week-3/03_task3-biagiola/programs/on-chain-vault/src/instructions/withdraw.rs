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
    // TODO: Add required accounts and constraints
    #[account(mut)]
    pub vault_authority: Signer<'info>,

    #[account(
        mut,
        seeds = [b"vault", vault_authority.key().as_ref()],
        bump,
        constraint = !vault.locked @ VaultError::VaultLocked
    )]
    pub vault: Account<'info, Vault>,

    pub system_program: Program<'info, System>,
}

pub fn _withdraw(ctx: Context<Withdraw>, amount: u64) -> Result<()> {
    // TODO: Implement withdraw functionality
    let vault = &mut ctx.accounts.vault;
    let vault_authority = &ctx.accounts.vault_authority;

    // Check if vault has enough balance (including rent exemption)
    let vault_balance = vault.to_account_info().lamports();
    let rent_exemption = Rent::get()?.minimum_balance(vault.to_account_info().data_len());
    
    require!(
        vault_balance >= amount.checked_add(rent_exemption).ok_or(VaultError::Overflow)?,
        VaultError::InsufficientBalance
    );

    // Transfer lamports from vault to vault authority
    **vault.to_account_info().try_borrow_mut_lamports()? -= amount;
    **vault_authority.to_account_info().try_borrow_mut_lamports()? += amount;

    // Emit withdraw event
    emit!(WithdrawEvent {
        amount,
        vault_authority: vault_authority.key(),
        vault: vault.key(),
    });

    Ok(())
}