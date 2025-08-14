use anchor_lang::prelude::*;
use crate::states::*;

#[derive(Accounts)]
pub struct SaveAddress<'info> {
    #[account(
        init, 
        payer = signer, 
        space = 8 + AddressData::INIT_SPACE,
        seeds = [b"user_address", signer.key().as_ref()],
        bump
    )]
    pub address_data: Account<'info, AddressData>,
    
    #[account(mut)]
    pub signer: Signer<'info>,
    
    pub system_program: Program<'info, System>,
}

pub fn save_address(ctx: Context<SaveAddress>) -> Result<()> {
    let address_data = &mut ctx.accounts.address_data;
    address_data.owner = ctx.accounts.signer.key();
    Ok(())
}