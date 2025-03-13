use anchor_lang::prelude::*;

declare_id!("4BtDbdzhLtz5AU4qgcse2WVq9eSDHGqCwRQFubqku63X");

#[program]
pub mod program_b {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    /// CHECK:
    pub pda_account:Signer<'info>,
}
