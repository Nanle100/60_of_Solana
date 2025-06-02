use anchor_lang::prelude::*;

declare_id!("EnRdy8E6rX5d9ByiFgTL5U1Jw23kryegJdW9kLFf7zZF");

#[program]
pub mod day1 {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        //msg!("Greetings from: {:?}", ctx.program_id);
         msg!("Hello, world!"); // **** NEW LINE HERE ****
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
