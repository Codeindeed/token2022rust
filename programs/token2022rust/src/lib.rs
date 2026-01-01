use anchor_lang::prelude::*;
use anchor_spl::token_interface::{TokenInterface, Mint, TokenAccount};
use anchor_spl::associated_token::AssociatedToken;

declare_id!("28MPfdpbBHx1dbBFy1g7Qj2Rkz6rzxMdQ146ZkcdXGgM");

#[program]
pub mod token2022rust {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct CreateMint<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,
    #[account(init,payer = signer, mint::decimals = 6, mint::authority = signer.key(), mint::token_program = token_program)]
    pub mint: Account<'info,Mint>,
    pub system_program: Program<'info,System>,
    pub token_program:Interface<'info,TokenInterface>,
}

#[derive(Accounts)]
pub struct Createssociatedtokenaccount<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,
    pub mint: Interface<'info, Mint>,
    #[account(
        init,
        payer = signer,
        associated_token::mint = mint,
        associated_token::authority = signer,
        associated_token::token_program = token_program
    )]
    pub associated_token: InterfaceAccount<'info, TokenAccount>,
    pub system_program: Program<'info, System>,
    pub token_program: Interface<'info, TokenInterface>,
    pub associated_token_program: Program<'info, AssociatedToken>,
}

#[derive(Accounts)]
pub struct CreateToken<'info>{
    #[account(mut)]
    pub signer:Signer<'info>,
    pub mint: InterfaceAccount<'info,Mint>,
    #[account(
        mut,
        token::mint = mint,
        token::authority = signer,
        token::token_program = token_program
    )]
    pub token: InterfaceAccount<'info, TokenAccount>,
    pub system_program: Program<'info, System>,
    pub token_program: Interface<'info, TokenInterface>,
}