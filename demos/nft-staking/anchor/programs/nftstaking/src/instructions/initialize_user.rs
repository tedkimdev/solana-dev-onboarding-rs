use anchor_lang::prelude::*;

use crate::state::user_account::UserAccount;

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(mut)]
    pub user: Signer<'info>,
    #[account(
        init,
        payer = user,
        seeds = [b"user".as_ref(), user.key().as_ref()],
        space = 8 + UserAccount::INIT_SPACE,
        bump,
    )]
    pub user_account: Account<'info, UserAccount>,
    pub system_program: Program<'info, System>,
}

impl<'info> Initialize<'info> {
    pub fn initialize_user(&mut self, bumps: &InitializeBumps) -> Result<()> {
        self.user_account.set_inner(UserAccount {
            points: 0,
            amount_staked: 0,
            bump: bumps.user_account,
        });

        Ok(())
    }
}