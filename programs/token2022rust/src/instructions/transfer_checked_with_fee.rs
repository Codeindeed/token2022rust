use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token_2022::spl_token_2022::{
        extension::{
            transfer_fee::TransferFeeConfig, BaseStateWithExtensions, StateWithExtensions,
        },
        state::Mint as MintState,
    },
    token_interface::{
        transfer_checked_with_fee, Mint, Token2022, TokenAccount, TransferCheckedWithFee,
    },
};

#[derive(Accounts)]
pub struct Transfer<'info>{
    #[account(mut)]
    pub sender: Signer<'info>,
    pub reciepient: SystemAccount<'info>,
    #[account(mut)]
    pub mint_account: Interface<'info, Mint>,
    #[account(mut,
    associated_token::mint = mint_account,
    associated_token::authority = reciepient,
    associated_token::token_program = token_program)] 
    pub reciepient_token_account: InterfaceAccount<'info, TokenAccount>,
    pub token_program: Program<'info, Token2022>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info, System>,
}

pub fn transfer_checked_with_fee(ctx: Context<Transfer>, amount: u64) -> Result<()> {
    transfer_checked_with_fee(
        CpiContext::new(
            ctx.accounts.token_program.to_account_info(),
            TransferCheckedWithfee{
                token_program_id: ctx.accounts.token_program.key(),
                source: ctx.accounts.sender.to_account_info().key(),
                mint: ctx.accounts.mint_account.to_account_info().key(),
                destination: ctx.accounts.reciepient_token_account.to_account_info().key(),
            },
        ),
        amount,
        decimals,
        fee,
    )?;
    Ok(());
}