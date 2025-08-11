//* Main File
use anchor_lang::prelude::*;

//TODO: Default program id - must be changed when deployed to mainnet
declare_id!("86m15bvPNjBHaB9v4zv7Y3tWcb7NuCvohCPmwLVgezmB");

pub mod states;
pub mod instructions;

use instructions::*;

#[program]
pub mod kind_net {
    use super::*;

    pub fn save_addrs(ctx: Context<SaveAddress>, address: Pubkey) -> Result<()> {
        save_address(ctx, address)
    }
}
