use anchor_lang::prelude::*;

declare_id!("3bTh4j1EvpQ8gCKbyGs6Fh8LEW7x33kkGY9ojhtGH25B");

#[program]
pub mod hello_cargo {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
