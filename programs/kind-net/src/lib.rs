//* Main File
use anchor_lang::prelude::*;

//TODO: Default program id - must be changed when deployed to mainnet
declare_id!("86m15bvPNjBHaB9v4zv7Y3tWcb7NuCvohCPmwLVgezmB");

#[program]
pub mod kind_net {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
