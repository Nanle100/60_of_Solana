use anchor_lang::prelude::*;

declare_id!("7hMx3yfR6JSec2vkyKNhdy3dnrA25jwTqpm5TsPMpLHp");

#[program]
pub mod day3 {
    use super::*;

  pub fn add(ctx: Context<NonEmptyAccountExample>, a: u64, b: u64) -> Result<()> {
  let sum = a + b;
  msg!("Sum is {}", sum);  
    Ok(())
}

pub fn sub(ctx: Context<Initialize>, a: u64, b: u64) -> Result<()> {
  let difference = a - b;
  msg!("Difference is {}", difference);  
    Ok(())
}

}

#[derive(Accounts)]
pub struct NonEmptyAccountExample<'info> {
    signer: Signer<'info>,
    another_signer: Signer<'info>,
}

#[derive(Accounts)]
pub struct Initialize {}


