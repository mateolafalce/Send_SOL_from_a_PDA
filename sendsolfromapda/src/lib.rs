use anchor_lang::prelude::*;

declare_id!("Ap1RcsWctAUWwMaWpwSd15bWtBQAgL5gcFnvSEXa3Tzh");

#[program]
pub mod sendsolfromapda {
    use super::*;

    pub fn initialize_pda(
        ctx: Context<CratePDA>
    ) -> Result<()> {
        let pda: &mut Account<PDAData> = &mut ctx.accounts.pda;
    let (_pda, bump) = Pubkey::find_program_address(&[b"send_one_sol", ctx.accounts.user.key().as_ref()], ctx.program_id);
    pda.bump = bump;
        Ok(())
    }
    pub fn send_sol_from_a_pda(
        ctx: Context<Transaction>,
        amount: u64
    ) -> Result<()> {
        **ctx.accounts.pda.to_account_info().try_borrow_mut_lamports()? -= amount;
        **ctx.accounts.to.try_borrow_mut_lamports()? += amount;
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