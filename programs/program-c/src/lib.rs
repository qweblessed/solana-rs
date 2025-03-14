use anchor_lang::{
    prelude::*,
    solana_program::{instruction::Instruction, program::invoke_signed},
};
use anchor_spl::token_interface::{Mint, TokenAccount, TokenInterface};
use jupiter_aggregator::program::Jupiter;
use std::str::FromStr;
mod instructions;

declare_id!("7vc3k64pE9wkxkmhNiK7HRyC7ymhPd9UBzuD11gtaWiQ");
declare_program!(jupiter_aggregator);

const VAULT_SEED: &[u8] = b"vault";

pub fn jupiter_program_id() -> Pubkey {
    Pubkey::from_str("JUP6LkbZbjS1jKKwapdHNy74zcZ3tLUZoi5QNyVTaV4").unwrap()
}

#[program]
pub mod program_c {    
    use super::*;

    pub fn swap(ctx: Context<Swap>, data: Vec<u8>) -> Result<()> {
        require_keys_eq!(*ctx.accounts.jupiter_program.key, jupiter_program_id());

        let accounts: Vec<AccountMeta> = ctx
            .remaining_accounts
            .iter()
            .map(|acc| {
                let is_signer = acc.key == &ctx.accounts.vault.key();
                AccountMeta {
                    pubkey: *acc.key,
                    is_signer,
                    is_writable: acc.is_writable,
                }
            })
            .collect();

        let accounts_infos: Vec<AccountInfo> = ctx
            .remaining_accounts
            .iter()
            .map(|acc| AccountInfo { ..acc.clone() })
            .collect();

        let signer_seeds: &[&[&[u8]]] = &[&[VAULT_SEED, &[ctx.bumps.vault]]];

        invoke_signed(
            &Instruction {
                program_id: ctx.accounts.jupiter_program.key(),
                accounts,
                data,
            },
            &accounts_infos,
            signer_seeds,
        )?;

        Ok(())
    }

    pub fn create_and_join_pool(ctx: Context<CreatePool>) -> Result<()> {
        let pool = &mut ctx.accounts.pool;

        pool.creator = ctx.program_id.key();
        pool.created_at = Clock::get()?.unix_timestamp as u64;
        pool.participants = Vec::new();

        msg!(
            "Pool created by {} at timestamp {}",
            pool.creator,
            pool.created_at
        );

        instructions::deposit_funds::deposit_funds(
            &ctx.accounts.signer,
            &ctx.accounts.pda_account,
            &ctx.accounts.system_program,
            &ctx.accounts.pool.key(),
            ctx.program_id,
        )?;

        Ok(())
    }

    pub fn join_pool(ctx: Context<JoinPool>) -> Result<()> {
        let pool = &mut ctx.accounts.pool;

        
        if pool.participants.len() >= 10 {
            return Err(ProgramError::Custom(0).into()); 
        }

        pool.participants.push(ctx.accounts.signer.key());

        msg!("{} joined the pool", ctx.accounts.signer.key());

        instructions::deposit_funds::deposit_funds(
            &ctx.accounts.signer,
            &ctx.accounts.pda_account,
            &ctx.accounts.system_program,
            &ctx.accounts.pool.key(),
            ctx.program_id,
        )?;

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


#[derive(Accounts)]
pub struct Swap<'info> {
    pub input_mint: InterfaceAccount<'info, Mint>,
    pub input_mint_program: Interface<'info, TokenInterface>,
    pub output_mint: InterfaceAccount<'info, Mint>,
    pub output_mint_program: Interface<'info, TokenInterface>,

    #[account(
      mut,
      seeds=[VAULT_SEED],
      bump
    )]
    pub vault: SystemAccount<'info>,

    #[account(
      mut,
      associated_token::mint=input_mint,
      associated_token::authority=vault,
      associated_token::token_program=input_mint_program,
    )]
    pub vault_input_token_account: InterfaceAccount<'info, TokenAccount>,

    #[account(
      mut,
      associated_token::mint=output_mint,
      associated_token::authority=vault,
      associated_token::token_program=output_mint_program,
    )]
    pub vault_output_token_account: InterfaceAccount<'info, TokenAccount>,

    pub jupiter_program: Program<'info, Jupiter>,
}