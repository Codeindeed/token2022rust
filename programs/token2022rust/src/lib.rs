use anchor_lang::prelude::*;

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
pub struct Initialize {}
