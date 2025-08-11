//* Data Structure

use anchor_lang::prelude::*;

#[account]
pub struct AddressData {
    pub owner: Pubkey,
    pub captured_address: Pubkey,
}