//* Data Structure

use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct AddressData {
    pub owner: Pubkey
}