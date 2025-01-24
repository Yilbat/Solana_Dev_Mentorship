use anchor_lang::prelude::*;

declare_id!("FeJEADiL5cbnkgVzmB8Kyj3xZuPHpDF572Z8i1uRQ7gj");

#[program]
pub mod eschrow {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }

    // buy function
    pub fn buy(ctx: Context<Buy>, product_name: String, amount: u64) -> Result<()> {
        Ok(())  // Signals successful transaction
    }

    // implement sell function
    pub fn sell(ctx: Context<Sell>, product_name: String, amount: u64)-> Result<()> {
        Ok(())  // Signals successful transaction
    }
    



#[derive(Accounts)]
pub struct Initialize {}

#[derive(Accounts)]
pub struct Buy {}

#[derive(Accounts)]
pub struct Sell {}
