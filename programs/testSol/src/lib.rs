use anchor_lang::prelude::*;
use anchor_lang::solana_program::entrypoint::ProgramResult;


// declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");
declare_id!("ATXhpcyyi4UdvXtRCkJZHqeuMs2mu9L9QvUoSxLPXDtk");

#[program]
pub mod test_sol {
    use super::*;
    pub fn initialize(_ctx: Context<Initialize>) -> ProgramResult {
        Ok(())
    }

    pub fn create(ctx: Context<Create>) -> ProgramResult {
        let counter = &mut ctx.accounts.counter;
        counter.count = 0;
        Ok(())
    }

    pub fn increment(ctx: Context<Increment>) -> ProgramResult {
        let counter = &mut ctx.accounts.counter;
        counter.count += 1;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}


#[derive(Accounts)]
pub struct Create<'info> {
    
    #[account(init, payer=user, space = 16+16)]
    pub counter: Account<'info, Counter>,
    
    #[account(mut)]
    pub user: Signer<'info>,
    
    pub system_program: Program<'info, System>,
}
#[account]
pub struct Counter {
    pub count: u64,
}

#[derive(Accounts)]
pub struct Increment<'info>{
    #[account(mut)]
    pub counter: Account<'info,Counter>
}
