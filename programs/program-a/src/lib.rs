use anchor_lang::prelude::*;
use program_b::program::ProgramB;
declare_id!("4VGfDBA9bzNQePeb5CHG3XMbBMuWXozKJF5uAWH6asue");

#[program]
pub mod program_a {
    use anchor_lang::solana_program::{
        program::{invoke, invoke_signed},
        system_instruction,
    };

    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);

        let pda_address = ctx.accounts.pda_account.key();
        let signer_address = ctx.accounts.signer.key();
        let bump = ctx.bumps.pda_account;

        let instruction =
            &system_instruction::transfer(&signer_address, &pda_address, 1_000_000_000);

        let account_infos = [
            ctx.accounts.pda_account.to_account_info(),
            ctx.accounts.signer.to_account_info(),
            ctx.accounts.system_program.to_account_info(),
        ];

        let signer_seeds: &[&[&[u8]]] = &[&[b"ackee", signer_address.as_ref(), &[bump]]];

        invoke_signed(instruction, &account_infos, signer_seeds)?;        

        let cpi_context = CpiContext::new(
            ctx.accounts.program_b.to_account_info(),
            program_b::cpi::accounts::Initialize {
                pda_account: ctx.accounts.pda_account.to_account_info(),
            }            
        );

        program_b::cpi::initialize(cpi_context)?;
        
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    /// CHECK: ackee
    #[account(
        mut,
        seeds = [b"ackee",signer.key().as_ref()],
        bump,
    )]
    pub pda_account: AccountInfo<'info>,
    #[account(mut)]
    pub signer: Signer<'info>,
    pub system_program: Program<'info, System>,
    pub program_b: Program<'info, ProgramB>,
}
