use anchor_lang::prelude::*;

declare_id!("MrtNanxhE1h81B3cPfBsGsfpL2uc7de5vpQ5uREWrrR");

#[program]
pub mod counter {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("First contract ever by Merto is deployed ! ");
        let counter:&mut Account<Counter> = &mut ctx.accounts.counter;
        counter.count = 0;
        Ok(())
    }
    pub fn update(ctx: Context<UpdateCtx>, data:u8) -> Result<()> {
        
        let counter:&mut Account<Counter> = &mut ctx.accounts.counter;
        counter.count = data;
        Ok(())
    }

}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(
        init,
        payer = payer,
        space = 8 + 1
    )]
    counter:Account<'info, Counter>,
    #[account(mut)]
    payer:Signer<'info>,
    system_program:Program<'info, System>
}
#[derive(Accounts)]
pub struct UpdateCtx<'info>{
    #[account(mut)]
    counter: Account<'info, Counter>,
    #[account(mut)]
    payer:Signer<'info>
}


#[account]
pub struct Counter{
    pub count:u8

}
