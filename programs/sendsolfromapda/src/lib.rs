use anchor_lang::prelude::*;

declare_id!("Gm9srn94QiQyfcz28y6g54F8zM6SwqgpPzNe7XMN91i8");

#[program]
pub mod sendsolfromapda {
    use super::*;

    pub fn initialize_pda(
        ctx: Context<CratePDA>,
        bump: u8
    ) -> Result<()> {
        let pda: &mut Account<PDAData> = &mut ctx.accounts.pda;
        pda.bump = bump;
        Ok(())
    }
    pub fn send_one_sol_from_a_pda(
        ctx: Context<Transaction>,
        amount: u64
    ) -> Result<()> {
        let _amount: u64 = amount - 953520;
        let from: &mut Account<PDAData> = &mut ctx.accounts.pda;
        let to: &mut AccountInfo = &mut ctx.accounts.to;
        **from.to_account_info().try_borrow_mut_lamports()? -= _amount;
        **to.try_borrow_mut_lamports()? += _amount;
        Ok(())
    }
}
#[derive(Accounts)]
pub struct CratePDA<'info> {
    #[account(init, seeds = [b"send_one_sol", user.key().as_ref()], bump, payer = user, space = 9)]
    pub pda: Account<'info, PDAData>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}
#[derive(Accounts)]
pub struct Transaction<'info> {
    /// CHECK: This is not dangerous because we don't read or write from this account
    #[account(mut, seeds = [b"send_one_sol", user.key().as_ref()], bump = pda.bump)]
    pub pda: Account<'info, PDAData>,
    /// CHECK: This is not dangerous because we don't read or write from this account
    #[account(mut)]
    pub to: AccountInfo<'info>,
    #[account(mut)]
    pub user: Signer<'info>,
}
#[account]
pub struct PDAData {
    bump: u8,
}