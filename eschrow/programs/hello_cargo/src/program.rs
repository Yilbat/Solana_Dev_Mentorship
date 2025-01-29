use anchor_lang::preclude::*;

declare_id!("program public key")

#[program]
pub mod counter_program{
    use super::*; 

    pub fn intialize(ctx:
    Context<Intialize>) -> Result<()>{
        let counter = &mut ctx.accounts.counter;
        counter.value = 0;
        ok(())
    }

    pub fn increment(ctx: Context<Increment>) -> Result<()>{
        let counter = &mut ctx.accounts.counter
        counter.value +=1
        msg!("Counter value is: {}", counter.value)
        ok(())
    }

    

}

#[derive(Accounts)]
    pub struct Intialize<'info>{
        #[account(init, payer = user, space = 8 + 8)]
        pub counter: Account<'info, Counter>,
        #[account(mut)]
        pub user: signer<'info>
        pub system_program: Program<'info, System>,
    }
#[derive(Accounts)]
    pub struct Increment<'info>{
        #[account(mut)]
        pub counter: Account<'info, Counter>,
    }

#[account]
pub struct Counter{
    pub value: u64
}