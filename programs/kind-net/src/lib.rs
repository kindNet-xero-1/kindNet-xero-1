//* Main File
use anchor_lang::prelude::*;

//TODO: Default program id - must be changed when deployed to mainnet
declare_id!("2pee52VCJhdyAhPdRbdWY3HHmTizv5f533SttsaniB4f");

pub mod errors;
pub mod instructions;
pub mod states;

use instructions::*;

#[program]
pub mod kind_net {
    use super::*;

    pub fn save_addrs(ctx: Context<SaveAddress>) -> Result<()> {
        save_address(ctx)
    }
}
