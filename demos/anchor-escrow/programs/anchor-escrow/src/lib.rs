use anchor_lang::prelude::*;

mod state;
pub mod instructions;

pub use state::*;
pub use instructions::*;

declare_id!("7i2Qo2rHxo4gqf2D2VS96Zqu5Xpa7xghruhVveZv5wms");

#[program]
pub mod anchor_escrow {
    use super::*;

    pub fn make(
        ctx: Context<Make>,
        seed: u8,
        receive_amount: u64,
        deposit_amount: u64,
    ) -> Result<()> {
        ctx.accounts.init_escrow_state(seed, receive_amount, ctx.bumps)?;
        ctx.accounts.deposit(deposit_amount)?;
        Ok(())
    }

    pub fn take(ctx: Context<Take>) -> Result<()> {
        ctx.accounts.taker_send_to_maker()?;
        ctx.accounts.taker_receive_and_close_escrow()?;
        Ok(())
    }

    pub fn refund(ctx: Context<Refund>) -> Result<()> {
        ctx.accounts.refund_and_close_vault()?;
        Ok(())
    }
}
