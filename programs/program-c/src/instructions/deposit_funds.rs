use anchor_lang::prelude::*;
use anchor_lang::solana_program::{
    program::{invoke},
    system_instruction,
};

pub(crate) fn deposit_funds<'info>(
    signer: &Signer<'info>,
    pda_account: &AccountInfo<'info>,
    system_program: &Program<'info, System>,
    pool_address: &Pubkey,
    program_id: &Pubkey,
) -> Result<()> {
    let seeds = &[b"pool_pda", pool_address.as_ref(), signer.key.as_ref()];
    let (pda_address, _bump_seed) = Pubkey::find_program_address(seeds, program_id);
    msg!("PDA Address: {}", pda_address);
    
    if pda_account.key() != pda_address {
        return Err(ProgramError::InvalidAccountData.into());
    }

    let instruction =
        system_instruction::transfer(&signer.key(), &pda_account.key(), 1_000_000_000);

    let account_infos = [
        pda_account.to_account_info(),
        signer.to_account_info(),
        system_program.to_account_info(),
    ];

    invoke(&instruction, &account_infos)?;
    Ok(())
}
