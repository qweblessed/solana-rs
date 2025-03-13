use anchor_lang::prelude::*;

declare_id!("7vc3k64pE9wkxkmhNiK7HRyC7ymhPd9UBzuD11gtaWiQ");

#[program]
pub mod program_c {
    use anchor_lang::solana_program::{
        program::{invoke},
        system_instruction,
    };

    use super::*;

    pub fn create_and_join_pool(ctx: Context<CreatePool>) -> Result<()> {
        let pool = &mut ctx.accounts.pool;

        pool.creator = ctx.program_id.key();
        pool.created_at = Clock::get()?.unix_timestamp as u64;
        pool.participants = Vec::new();

        msg!("Pool created by {} at timestamp {}", pool.creator, pool.created_at);

        let pool_address = ctx.accounts.pool.key();
        let signer_address = ctx.accounts.signer.key();
   
        let seeds = &[b"pool_pda", pool_address.as_ref(), signer_address.as_ref()];
        let (pda_address, _bump_seed) = Pubkey::find_program_address(seeds, &ctx.program_id);
        msg!("PDA Address: {}", pda_address);

        let instruction = system_instruction::transfer(
            &ctx.accounts.signer.key(),
            &pda_address,
            1_000_000_000,
        );

        let account_infos = [
            ctx.accounts.pda_account.to_account_info(),
            ctx.accounts.signer.to_account_info(),
            ctx.accounts.system_program.to_account_info(),
        ];
        
        invoke(
            &instruction,
            &account_infos,
        )?;
        Ok(())
    }

    pub fn join_pool(ctx: Context<JoinPool>) -> Result<()> {
        let pool = &mut ctx.accounts.pool;

        // Check if the pool is full (limit is 10 participants)
        if pool.participants.len() >= 10 {
            return Err(ProgramError::Custom(0).into());  // Custom error: Pool is full
        }

        // Add the new participant
        pool.participants.push(ctx.accounts.signer.key());

        msg!("{} joined the pool", ctx.accounts.signer.key());

        // Transfer funds to the PDA
        let pool_address = ctx.accounts.pool.key();
        let signer_address = ctx.accounts.signer.key();
   
        let seeds = &[b"pool_pda", pool_address.as_ref(), signer_address.as_ref()];
        let (pda_address, _bump_seed) = Pubkey::find_program_address(seeds, &ctx.program_id);
        msg!("PDA Address: {}", pda_address);

        let instruction = system_instruction::transfer(
            &ctx.accounts.signer.key(),
            &pda_address,
            1_000_000_000,
        );

        let account_infos = [
            ctx.accounts.pda_account.to_account_info(),
            ctx.accounts.signer.to_account_info(),
            ctx.accounts.system_program.to_account_info(),
        ];
        
        invoke(&instruction, &account_infos)?;
        Ok(())
    }


}


#[derive(Accounts)]
pub struct CreatePool<'info> {
    #[account(init, payer = signer, space = 8 + 32 + 8 + 32)]
    pub pool: Account<'info, Pool>,
    #[account(
        mut,
        seeds = [b"pool_pda", pool.key().as_ref(), signer.key().as_ref()],
        bump
    )]    
    /// CHECK: qwe
    pub pda_account: AccountInfo<'info>,
    #[account(mut)]
    pub signer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct JoinPool<'info> {
    #[account(mut)]
    pub pool: Account<'info, Pool>,
    #[account(
        mut,
        seeds = [b"pool_pda", pool.key().as_ref(), signer.key().as_ref()],
        bump
    )]
    /// CHECK: qwe
    pub pda_account: AccountInfo<'info>,
    #[account(mut)]
    pub signer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct Pool {
    pub creator: Pubkey, // Now the creator is the program's public key
    pub created_at: u64, // Timestamp of pool creation
    pub participants: Vec<Pubkey>,
}