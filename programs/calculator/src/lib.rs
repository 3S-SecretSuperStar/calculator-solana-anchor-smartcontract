use anchor_lang::prelude::*;

declare_id!("DqkjQjDoQD69hnLHxWj7Q5wwNG5oByq4fDQF9LgZ8LZN");

#[program]
pub mod calculator {
    use super::*;
	pub fn create(ctx: Context<Create>, init_message: String) -> Result<()> {
		let calculator = &mut ctx.accounts.calculator;
		calculator.greeting = init_message;
		Ok(())
	}

	pub fn add(ctx: Context<Addition>, num1: i64, num2: i64) -> Result<()> {
		let calculator = &mut ctx.accounts.calculator;
		calculator.result = num1 + num2;
		Ok(())
	}

	pub fn sub(ctx: Context<Substraction>, num1: i64, num2: i64) -> Result<()> {
		let calculator = &mut ctx.accounts.calculator;
		calculator.result = num1 - num2;
		Ok(())
	}

	pub fn mul(ctx: Context<Multiplication>, num1: i64, num2: i64) -> Result<()> {
		let calculator = &mut ctx.accounts.calculator;
		calculator.result = num1 * num2;
		Ok(())
	}

	pub fn div(ctx: Context<Division>, num1: i64, num2: i64) -> Result<()> {
		let calculator = &mut ctx.accounts.calculator;
		calculator.result = num1 / num2;
		calculator.remainder = num1 % num2;
		Ok(())
	}
}

#[account]
#[derive(Default)]
pub struct Calculator {
	pub greeting: String,
	pub result: i64,
	pub remainder: i64,
}

#[derive(Accounts)]
pub struct Addition<'info> {
	#[account(mut)]
	pub calculator: Account<'info, Calculator>
}

#[derive(Accounts)]
pub struct Substraction<'info> {
	#[account(mut)]
	pub calculator: Account<'info, Calculator>
}

#[derive(Accounts)]
pub struct Multiplication<'info> {
	#[account(mut)]
	pub calculator: Account<'info, Calculator>
}

#[derive(Accounts)]
pub struct Division<'info> {
	#[account(mut)]
	pub calculator: Account<'info, Calculator>
}

#[derive(Accounts)]
pub struct Create<'info> {
	#[account(init, payer=user, space = 264)]
	pub calculator: Account<'info, Calculator>,
	#[account(mut)]
	pub user: Signer<'info>,
 	pub system_program: Program<'info, System>
}
