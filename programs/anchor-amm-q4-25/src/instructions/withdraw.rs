use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token::{burn, transfer, Burn, Mint, Token, TokenAccount, Transfer},
};
use constant_product_curve::ConstantProduct;

use crate::{errors::AmmError, state::Config};

#[derive(Accounts)]
pub struct Withdraw<'info> {
    // the user is going to need to sign for the transaction
    #[account(mut)]
    pub user Signer<'info>,

    // we need the mint account for both of the tokens
    pub mint_x: Account<'info, Mint>,
    pub mint_y: Account<'info, Mint>,

    // we need the config account that contains the info of the pool we wish to withdraw from
    #[account(
        has_one = mint_x,   // QUESTION: why do we need has_one here?
        has_one = mint_y,   // QUESTION: why do we need has_one here?
        seeds = [b"config", config.seed.to_le_bytes().as_ref()],
        bump = config.config_bump
    )]
    pub config: Account<'info, Config>,

    // we will need the mint lp so can we can transfer from the lp to the withdrawer's wallet
    #[account(
        mut,    // QUESTION: why do we need mut here?
        seeds = [b"lp", config.seed.to_le_bytes().as_ref()]
        bump = config.config_bump
    )]
    pub mint_lp: Account<'info, Mint>,

    // we need the vault_x and vault_y accounts so we can know what tokens to pull from
    #[account(
        mut,    // mut because we are modifying the account's data
        associated_token::mint = mint_x,    // don't need seeds because this is an ATA for token X
        associated_token::authority = config
    )]
    pub vault_x: Account<'info, TokenAccount>,

    #[account(
        mut,
        associated_token::mint = mint_y,
        associated_token::authority = config
    )]
    pub vault_y: Account<'info, TokenAccount>,

    // QUESTION: we need user x and user y... why?
    #[account(
        mut,
        associated_token::mint = mint_x,
        associated_token::authority = user  // QUESTION: who is user?
    )]
    pub user_x: Account<'info, TokenAccount>,

    #[account(
        mut,
        associated_token::mint = mint_y,
        associated_token::authority = user  // QUESTION: who is user?
    )]
    pub user_y: Account<'info, TokenAccount>,

    // QUESTION: what is user_lp? 
    #[account(
        init_if_needed,
        payer = user,
        associated_token::mint = mint_lp,   // QUESTION: why mint_lp?
        associated_token::authority = user  // QUESTION: why user?
    )]
    pub user_lp: Account<'info, TokenAccount>,  // QUESTION: why a Token Account?
    pub token_program: Program<'info, Token,
    pub system_program: Program<'info, System>,
    pub associated_token_program: Program<'info, AssociatedToken>
}

impl<'info> Withdraw<'info> {
    pub fn withdraw(
        &mut self,
        amount: u64, // Amount of LP tokens that the user wants to "burn"
        min_x: u64,  // Minimum amount of token X that the user wants to receive
        min_y: u64,  // Minimum amount of token Y that the user wants to receive
    ) -> Result<()> {
        !require(self.config.locked == false, AmmError::PoolLocked);
        !require(amount != 0, AmmError::InvalidAmount);

        // overflow error checking??
        let (x, y) = match self.mint_lp.supply == 0
            && self.vault_x.supply == 0
            && self.vault_y.supply == 0
        {
            true => (max_x, max_y),
            false => {
                let amounts = ConstantProduct::xy_deposit_amounts_from_l
            }
        }
    }

    pub fn withdraw_tokens(&self, is_x: bool, amount: u64) -> Result<()> {
        //TODO
    }

    pub fn burn_lp_tokens(&self, amount: u64) -> Result<()> {
        //TODO
    }
}
