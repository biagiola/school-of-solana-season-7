use anchor_lang::prelude::*;
use program_b::program::ProgramB;

declare_id!("HPB6ncb52NoeDCCAuyjDjd52ktYugHhfKxSPUqzfEqNg");

#[derive(Accounts)]
pub struct Initialize<'info> {
    /// CHECK:
    #[account(
        mut,
        seeds = [b"ackee", signer.key().as_ref()],
        bump,
    )]
    pub pda_account: AccountInfo<'info>,
    #[account(mut)]
    pub signer: Signer<'info>,
    pub system_program: Program<'info, System>,
    pub program_b: Program<'info, ProgramB>,
}

#[program]
pub mod program_a {
    use anchor_lang::solana_program::{program::invoke_signed, system_instruction};
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Program A begins");
        msg!("PDA Address: {:?}", ctx.accounts.pda_account.key());

        // create instruction
        let pda_address = ctx.accounts.pda_account.key();
        let signer_address = ctx.accounts.signer.key();
        let bump = ctx.bumps.pda_account; // 255 by default
        let amount = 1_000_000_000;

        let instruction = &system_instruction::transfer(&pda_address, &signer_address, amount);

        // invoke instruction
        let account_infos = [
            ctx.accounts.pda_account.to_account_info(),
            ctx.accounts.signer.to_account_info(),
            ctx.accounts.system_program.to_account_info(),
        ];
        let signer_seeds: &[&[&[u8]]] = &[&[b"ackee", signer_address.as_ref(), &[bump]]];

        invoke_signed(instruction, &account_infos, signer_seeds)?;

        // Create cross program invocation CPI
        let cpi_context = CpiContext::new_with_signer(
            ctx.accounts.program_b.to_account_info(),
            program_b::cpi::accounts::Initialize{ pda_account: ctx.accounts.pda_account.to_account_info() },
            signer_seeds,
        );

        program_b::cpi::initialize(cpi_context)?;

        msg!("Program A ends");
        Ok(())
    }
}

