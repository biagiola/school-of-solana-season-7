use anchor_lang::prelude::*;

declare_id!("FBgf9LtNm1bSvU92Bkwk3afCVn3nqZcy2gNVPKSpU9C4");

#[derive(Accounts)]
pub struct Initialize<'info> {
    pub pda_account: Signer<'info>,
}

#[program]
pub mod program_b {
    use super::*;

    pub fn initialize(_ctx: Context<Initialize>) -> Result<()> {
        msg!("Program B begins");
        msg!("Program B ends");
        Ok(())
    }
}
