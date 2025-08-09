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
    // TODO: Add required accounts and constraints
    #[account(mut)]
    pub user: Signer<'info>,

    #[account(
        mut,
        seeds = [b"vault", vault.vault_authority.as_ref()],
        bump,
        constraint = !vault.locked @ VaultError::VaultLocked
    )]
    pub vault: Account<'info, Vault>,

    pub system_program: Program<'info, System>,
}

pub fn _deposit(ctx: Context<Deposit>, amount: u64) -> Result<()> {
    // TODO: Implement deposit functionality
    let user = &ctx.accounts.user;
    let vault = &ctx.accounts.vault;

    // Check if user has enough balance (including rent exemption)
    let user_balance = user.lamports();
    let rent_exemption = Rent::get()?.minimum_balance(0); // Minimum balance for user account
    
    require!(
        user_balance >= amount.checked_add(rent_exemption).ok_or(VaultError::Overflow)?,
        VaultError::InsufficientBalance
    );

    // Transfer lamports from user to vault using CPI
    let transfer_instruction = transfer(
        &user.key(),
        &vault.key(),
        amount,
    );

    invoke(
        &transfer_instruction,
        &[
            user.to_account_info(),
            vault.to_account_info(),
            ctx.accounts.system_program.to_account_info(),
        ],
    )?;

    // Emit deposit event
    emit!(DepositEvent {
        amount,
        user: user.key(),
        vault: vault.key(),
    });

    Ok(())
}