use anchor_lang::prelude::*;

declare_id!("HC15hMijkiBo6hFZaqXLJ8xhY57nQmLjbXCbF4DyBCSF");

#[program]
pub mod structure {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
