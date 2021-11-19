use anchor_lang::prelude::*;
use anchor_lang::solana_program::{program::invoke, system_instruction};

declare_id!("7aCUbFSGhaXtdAsZzmZKFhaHk3KJmCHrPUASc5mL4iHx");

#[program]
mod basic_1 {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>, data: u64) -> ProgramResult {
        let cost = 10000000000000000000;
        let my_account = &mut ctx.accounts.my_account;
        let user = &mut ctx.accounts.user;
        my_account.data = data;

        msg!("lamports from mike {}", user.to_account_info().lamports());
        if user.lamports() < cost {
            return Err(ErrorCode::NotEnoughSOL.into());
        }

        invoke(
            &system_instruction::transfer(
                &ctx.accounts.user.key,       //311.216876674
                ctx.accounts.destination.key, // 1525
                cost,
            ),
            &[
                ctx.accounts.user.to_account_info(),
                ctx.accounts.destination.to_account_info(),
                ctx.accounts.system_program.to_account_info(),
            ],
        )?;

        Ok(())
    }

    pub fn update(ctx: Context<Update>, data: u64) -> ProgramResult {
        let my_account = &mut ctx.accounts.my_account;
        my_account.data = data;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(init, payer = user, space = 8 + 8)]
    pub my_account: Account<'info, MyAccount>,
    #[account(mut)]
    pub user: Signer<'info>,
    #[account(mut)]
    pub destination: UncheckedAccount<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Update<'info> {
    #[account(mut)]
    pub my_account: Account<'info, MyAccount>,
}

#[account]
pub struct MyAccount {
    pub data: u64,
}

#[error]
pub enum ErrorCode {
    #[msg("Not enough SOL. A slab costs 1 SOL.")]
    NotEnoughSOL,
}
