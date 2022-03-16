use anchor_lang::prelude::*;

declare_id!("EmCT4huMZ1HuWMeNgEMMwaUkBBWfYQFpYFTxhFWNY17m");

#[program]
pub mod referral {
    use super::*;
    pub fn initialize(ctx: Context<Initialize>) -> ProgramResult {
        ctx.accounts.authority.authority = ctx.accounts.admin.key();
        Ok(())
    }
    pub fn add_referral(
        ctx: Context<AddRefferal>,
        code: String,
        vesting_pubkey: Pubkey,
    ) -> ProgramResult {
        if ctx.accounts.admin.key() != ctx.accounts.authority.key() {
            return Err(ErrorCode::NotAdmin.into());
        }
        ctx.accounts.referer.promo_code = code;
        ctx.accounts.referer.vesting_pubkey = vesting_pubkey;
        Ok(())
    }
    pub fn check_referral(ctx: Context<CheckReferral>) -> ProgramResult {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(init, payer=admin, space=Authority::space())]
    pub authority: Account<'info, Authority>,
    #[account(mut, signer)]
    pub admin: AccountInfo<'info>,
    pub system_program: AccountInfo<'info>,
}

#[derive(Accounts)]
#[instruction(text: String)]
pub struct AddRefferal<'info> {
    #[account(init, payer=admin, space=ReferralAccount::space(&text))]
    pub referer: Account<'info, ReferralAccount>,
    #[account(mut, signer)]
    pub admin: AccountInfo<'info>,
    #[account(has_one = authority)]
    pub authority: Account<'info, Authority>,
    pub beneficiary: AccountInfo<'info>,
    pub system_program: AccountInfo<'info>,
    // pub rent: Sysvar<'info, Rent>,
    // pub token_program: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct CheckReferral<'info> {
    #[account(init, payer=admin, space=Authority::space())]
    pub authority: Account<'info, Authority>,
    #[account(mut, signer)]
    pub admin: AccountInfo<'info>,
    pub system_program: AccountInfo<'info>,
}

#[account()]
pub struct Authority {
    pub authority: Pubkey,
}
impl Authority {
    fn space() -> usize {
        // discriminator
        8 +
        // pubkey
        std::mem::size_of::<Pubkey>()
    }
}

#[account()]
pub struct ReferralAccount {
    pub promo_code: String,
    pub vesting_pubkey: Pubkey,
}

impl ReferralAccount {
    fn space(text: &str) -> usize {
        // discriminator
        8 +
        // String
        4 + text.len()+
        // pubkey
        std::mem::size_of::<Pubkey>()
    }
}

#[error]
pub enum ErrorCode {
    #[msg("Seller provided isn't same as proposal")]
    NotAdmin,
}
