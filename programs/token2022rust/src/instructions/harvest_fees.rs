use anchor_lang::prelude::*;
use anchor_spl::token_interface::{
    harvest_withheld_tokens_to_mint, HarvestWithheldTokensToMint, withdraw_withheld_tokens_from_mint, WithdrawWithheldTokensFromMint, Mint, Token2022, TokenAccount,
};

#[derive(Accounts)]
pub struct HarvestandWithdrawFees<'info>{
    pub authority: Signer<'info>,
    #[account(mut)]
    pub mint_account: Interface<'info, Mint>,
    #[account(mut)]
    pub token_account: InterfaceAccount<'info, TokenAccount>,
    pub token_program: Program<'info, Token2022>,
   
}

pub fn harvest_and_withdraw_fees(ctx: Context<'_,'_,'info, 'info, HarvestandWithdrawFees<'info>>) -> Result<()> {

    let sources = ctx.remaining_accounts.iter().filter_map(|acc| {
        InterfaceAccount::<TokenAccount>::try_from(acc).ok().filter(|toke_account| toke_account.mint == ctx.accounts.mint_account.key()).map(|_| acc.to_account_info().key())
    }).collect::<Vec<_>>();
    harvest_withheld_tokens_to_mint(
        CpiContext::new(
            ctx.accounts.token_program.to_account_info(),
            HarvestWithheldTokensToMint{
                token_program_id: ctx.accounts.token_program.key(),
                mint: ctx.accounts.mint_account.to_account_info().key(),
            },
        ),
        sources,
    )?;
    withdraw_withheld_tokens_from_mint(
        CpiContext::new(
            ctx.accounts.token_program.to_account_info(),
            WithdrawWithheldTokensFromMint{
                token_program_id: ctx.accounts.token_program.key(),
                mint: ctx.accounts.mint_account.to_account_info().key(),
            },
        ),
        ctx.accounts.token_account.to_account_info().key(),
    )?;
    Ok(());
}

pub struct TransferFeeConfig {
    pub transfer_fee_config_authority: Pubkey,
    pub withdraw_withheld_authority: Pubkey,
    pub withheld_amount: u64,
    pub older_transfer_fee: TransferFee,
    pub newer_transfer_fee: TransferFee,
}
 
pub struct TransferFee {
    pub epoch: u64,
    pub maximum_fee: u64,
    pub transfer_fee_basis_point: u16,
}