use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token::{close_account, transfer_checked, CloseAccount, TransferChecked},
    token_interface::{Mint, TokenAccount, TokenInterface},
};

use crate::EscrowState;

#[derive(Accounts)]
pub struct Take<'info> {
    #[account(mut)]
    pub taker: Signer<'info>,

    #[account(
        address = escrow.mint_a,
    )]
    pub mint_a: InterfaceAccount<'info, Mint>,
    #[account(
        address = escrow.mint_b,
    )]
    pub mint_b: InterfaceAccount<'info, Mint>,

    // create the associated tokens for maker(mint_b), for taker (mint_a)
    #[account(
        init_if_needed,
        payer = taker,
        associated_token::mint = mint_a,
        associated_token::authority = taker,
    )]
    pub taker_mint_a_ata: InterfaceAccount<'info, TokenAccount>,
    // for Taker, mint-b is assumed to be created already
    #[account(
        mut,
        associated_token::mint = mint_b,
        associated_token::authority = taker,
    )]
    pub taker_mint_b_ata: InterfaceAccount<'info, TokenAccount>,

    #[account(
        init_if_needed,
        payer = taker,
        associated_token::mint = mint_b,
        associated_token::authority = maker,
    )]
    pub maker_mint_b_ata: InterfaceAccount<'info, TokenAccount>,
    /// CHECK: This is safe as we're only using it as a key.
    pub maker: AccountInfo<'info>,

    #[account(
        mut,
        has_one = mint_a,
        has_one = mint_b,
        seeds = [b"escrow", escrow.maker.as_ref(), escrow.seed.to_le_bytes().as_ref()],
        bump = escrow.bump,
        close = taker
    )]
    pub escrow: Account<'info, EscrowState>,

    #[account(
        mut,
        associated_token::mint = mint_a,
        associated_token::authority = escrow,
    )]
    pub escrow_vault: InterfaceAccount<'info, TokenAccount>,

    pub system_program: Program<'info, System>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub token_program: Interface<'info, TokenInterface>,
}

impl<'info> Take<'info> {
    pub fn taker_send_to_maker(&mut self) -> Result<()> {
        // taker transfer token b to maker
        let cpi_program = self.token_program.to_account_info();

        let cpi_accounts = TransferChecked {
            from: self.taker_mint_b_ata.to_account_info(),
            mint: self.mint_b.to_account_info(),
            to: self.maker_mint_b_ata.to_account_info(),
            authority: self.taker.to_account_info(),
        };

        let cpi_context = CpiContext::new(cpi_program, cpi_accounts);

        transfer_checked(
            cpi_context,
            self.escrow.receive_amount,
            self.mint_b.decimals,
        )?;

        Ok(())
    }

    // transfer the token a from the escrow vault to the taker
    pub fn taker_receive_and_close_escrow(&mut self) -> Result<()> {
        let cpi_program = self.token_program.to_account_info();

        let cpi_accounts = TransferChecked {
            from: self.escrow_vault.to_account_info(),
            mint: self.mint_a.to_account_info(),
            to: self.taker_mint_a_ata.to_account_info(),
            authority: self.escrow.to_account_info(),
        };

        let seed_bytes = self.escrow.seed.to_le_bytes();
        let seeds = &[
            b"escrow",
            self.escrow.maker.as_ref(),
            seed_bytes.as_ref(),
            &[self.escrow.bump],
        ];

        let signers_seeds = &[&seeds[..]];

        let cpi_ctx = CpiContext::new_with_signer(cpi_program, cpi_accounts, signers_seeds);

        transfer_checked(cpi_ctx, self.escrow_vault.amount, self.mint_a.decimals)?;

        // the closing escrow logic here
        let accounts_close = CloseAccount {
            account: self.escrow_vault.to_account_info(),
            destination: self.taker.to_account_info(),
            authority: self.escrow.to_account_info(),
        };

        let close_cpi_context = CpiContext::new_with_signer(
            self.token_program.to_account_info(),
            accounts_close,
            signers_seeds,
        );
        close_account(close_cpi_context)?;
        Ok(())
    }
}
