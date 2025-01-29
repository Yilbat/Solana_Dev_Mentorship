#[program]
pub mod eschrow {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        const name: &str = "ThankGod"; // string literal type
        const age: u64 = 50; //  unsigned integer type
        const iq: f32 = 0.5; // float type
        let friends: Vec<T> = Vec::new(); // vector type
        const skill: String = String::new("Solana Development"); // string type
        const students: [&str; 6] = ["ThankGod", "Agnes", "David", "Ola", "Abdul", "Isaac"]; //Array Type
        const is_good: bool = true; //Boelan type

        Ok(())
    }

    
    // implement buy function
    pub fn buy(ctx: Context<Buy>, product_name: String, amount: u64) -> Result<()> {
        Ok(())
    }


    // implement sell function
    pub fn sell(ctx: Context<Sell>, product_name: String, amount: u64) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {
    #[account(
        init, 
        payer = user,
        space = 8 + 32
    )]

    user: Signer<'info>,
}

#[derive(Accounts)]
pub struct Buy {}

#[derive(Accounts)]
pub struct Sell {}