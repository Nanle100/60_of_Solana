use anchor_lang::prelude::*;

declare_id!("2netPD5S3fqsJRYskBRerDMNZiPJk6H8MKYGH6w1DdV3");

#[program]
pub mod calculator {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }

    pub fn addition(ctx: Context<Initialize>, a: u64, b:u64) -> Result<()> {
        let answer_one: u64 = a.checked_add(b).unwrap();
        msg!("Answer for addition is {}", answer_one);
        Ok(())
    }

    pub fn substraction(ctx: Context<Initialize>, a: u64, b:u64) -> Result<()> {
        let answer_one: u64 = a.checked_sub(b).unwrap();
        msg!("Answer for substraction is {}", answer_one);
       Ok(())
    }

     pub fn multiplication(ctx: Context<Initialize>, a: u64, b:u64) -> Result<()> {
        let answer_one: u64 = a.checked_mul(b).unwrap();
        msg!("Answer for substraction is {}", answer_one);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
