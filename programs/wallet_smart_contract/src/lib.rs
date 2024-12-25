use anchor_lang::prelude::*;

declare_id!("3dLxZCxZmnitL9dkZLX9UZbNg1kGydbeciUd4HN4c22k");
//"3dLxZCxZmnitL9dkZLX9UZbNg1kGydbeciUd4HN4c22k"
#[program]
pub mod wallet_smart_contract {
    use super::*;

    // Initiallize new wallet with user Key
    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id); 
        let wallet = &mut ctx.accounts.wallet;
        wallet.owner = *ctx.accounts.user.key;
        wallet.balance = 0;
        Ok(())
    }

    pub fn deposit(ctx: Context<Deposit>, amount: u64) -> Result<()> {
        let wallet = &mut ctx.accounts.wallet;
        let user = &ctx.accounts.user;

        // Transfer tokens from the user to the wallet account
        let cpi_context = CpiContext::new(
            ctx.accounts.system_program.to_account_info(),
            anchor_lang::system_program::Transfer {
                from: user.to_account_info(),
                to: wallet.to_account_info(),
            },
        );
        anchor_lang::system_program::transfer(cpi_context, amount)?;

        wallet.balance += amount;
        Ok(())
    }

    pub fn withdraw(ctx: Context<Withdraw>, amount: u64) -> Result<()> {
        let wallet = &mut ctx.accounts.wallet;
        let owner = &ctx.accounts.owner;

        if wallet.balance < amount {
            return Err(ErrorCode::InsufficientFunds.into());
        }

        // Transfer tokens from the wallet account to the user
        **wallet.to_account_info().try_borrow_mut_lamports()? -= amount;
        **owner.to_account_info().try_borrow_mut_lamports()? += amount;

        wallet.balance -= amount;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(init, payer = user, space = 8 + 32 + 8)]
    pub wallet: Account<'info, Wallet>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Deposit<'info> {
    #[account(mut)]
    pub wallet: Account<'info, Wallet>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Withdraw<'info> {
    #[account(mut, has_one = owner)]
    pub wallet: Account<'info, Wallet>,
    #[account(mut)]
    pub owner: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct Wallet {
    pub owner: Pubkey,
    pub balance: u64,
}

#[error_code]
pub enum ErrorCode {
    #[msg("Insufficient funds for withdrawal")]
    InsufficientFunds,
}
