use anchor_lang::prelude::*;
use vesting::cpi::accounts::{InitializeVesting, Revoke, Withdraw};
use vesting::program::Vesting;
use vesting::{self, VestingAccount};

use referral::cpi::accounts::CheckReferral;
use referral::program::Referral;
use referral::{self, ReferralAccount};

declare_id!("FvzWwerZcSBbxb6HVjXoYU6qWt4tfX9sYaqdrYU1K2ex");

#[program]
pub mod presale {
    use super::*;
    pub fn initialize(
        ctx: Context<Initialize>,
        amount_to_deposit: u64,
        start_of_sale: i64,
        conversion_rate: u64,
        min_invested: u64,
        max_invested: u64,
    ) -> ProgramResult {
        /// minimum money / max ?
        /// whitelist ?
        ///
        ///
        let cpi_vesting = ctx.accounts.vesting_program.to_account_info();
        let cpi_referral = ctx.accounts.referral_program.to_account_info();

        let cpi_vesting_accounts = VestingAccount {
            beneficiary: Pubkey,
            /// The timestamp for when the lock ends and vesting begins
            start_ts: i64,
            /// The timestamp for when the cliff ends (vesting happens during cliff!)
            cliff_ts: i64,
            /// The duration of the vesting period
            duration: i64,
            /// Whether this vesting account is revocable
            revocable: bool,
            /// Owner that can revoke the account
            owner: Pubkey,
            /// The mint of the SPL token locked up.
            mint: Pubkey,
            /// Total amount to be vested
            total_deposited_amount: u64,
            /// Amount that has been released
            released_amount: u64,
            /// The account is revoked
            revoked: bool,
        };
        let cpi_referral_accounts = ReferralAccount {
            /// Code to check if exist
            promo_code: String,
            /// Where to add the funds for the referrer
            vesting_pubkey: Pubkey,
        };

        let cpi_vesting_ctx = CpiContext::new(cpi_vesting, cpi_vesting_accounts);
        let cpi_referral_ctx = CpiContext::new(cpi_referral, cpi_referral_accounts);

        vesting::cpi::initialize_vesting(
            cpi_vesting_ctx,
            amount_to_deposit,
            start_of_sale,
            cliff_ts: i64,
            duration: i64,
            revocable: bool,
        );

        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    pub vesting_program: Program<'info, Vesting>,
    pub referral_program: Program<'info, Referral>,
    pub whitelist: Account<'info,Whitelist>,
}

#[account()]
pub struct Whitelist {
    pub authority: Pubkey,
}