use anchor_lang::prelude::*;
use anchor_spl::token_interface::{transfer_fee_set, Mint, Token2022, TransferFeeSetTransferFee};



#[derive(Accounts)]
pub struct UpdateFee<'info> {
    pub authority: Signer<'info>,
    #[account(mut)]
    pub mint_account: InterfaceAccount<'info, Mint>,
    pub token_program: Program<'info, Token2022>,
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

pub fn update_fee(
    ctx: Context<UpdateFee>,
    transfer_fee_basis_points: u16,
    maximum_fee: u64,
) -> Result<()> {
    transfer_fee_set(
        CpiContext::new(
            ctx.accounts.token_program.to_account_info(),
            TransferFeeSetTransferFee {
                token_program_id: ctx.accounts.token_program.to_account_info(),
                mint: ctx.accounts.mint_account.to_account_info(),
                authority: ctx.accounts.authority.to_account_info(),
            },
        ),
        transfer_fee_basis_points, // transfer fee basis points (% fee per transfer)
        maximum_fee,               // maximum fee (maximum units of token per transfer)
    )?;
    Ok(())
}

