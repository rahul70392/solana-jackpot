use anchor_lang::{prelude::*, solana_program::system_program};

declare_id!("DP1BGMQdhhTE6CehdvhgQTBZe8mtA4RvmcAg8DQ6oxkd");


#[program]
pub mod solana_jackpot {
    use super::*;

        use anchor_lang::solana_program::{
        lamports,
        program::{invoke, invoke_signed},
        system_instruction::{transfer , assign_with_seed, assign}
    };

    pub fn initialize_bet(ctx: Context<InitializeBet>, betid: u32) -> Result<()> {
        let betaccount: &mut Account<BetAccount> = &mut ctx.accounts.bet;

        let clock: Clock = Clock::get().unwrap();
        let bet_id : u32 = betid;
        let mut vec = Vec::new();
        
        betaccount.bet_id = bet_id;
        betaccount.bet_result = None;
        betaccount.bettor_list = vec.clone();
        betaccount.timestamp = clock.unix_timestamp;
        betaccount.vault_pda = *ctx.accounts.vault_pda_account.to_account_info().key;
        // betaccount.betstate = BetState::Started;

        Ok(())
    }

    pub fn place_bet(ctx: Context<PlaceBet>, betid: u32, betAmount: u64) -> Result<()> {
        let betAccount: &mut Account<BetAccount> = &mut ctx.accounts.bet;
        let vault_pda_account: &mut Account<BetVaultAccount> = &mut ctx.accounts.vault_pda_account;

        let transfer_instruction = &transfer(
                        &ctx.accounts.bettor.key,
                        &vault_pda_account.to_account_info().key,
                        betAmount,
                    );
                    msg!("betting with {} lamports", betAmount);
                    invoke(
                        transfer_instruction,
                        &[
                            ctx.accounts.bettor.to_account_info(),
                            vault_pda_account.to_account_info(),       
                        ]
                    );

        Ok(())
    }
}


#[derive(Accounts)]
pub struct InitializeBet<'info> {
        #[account(init,payer = admin,seeds=[b"seed"],bump,space = BetAccount::LEN,)]
        pub bet: Account<'info, BetAccount>,

        #[account(mut)]
        pub admin: Signer<'info>, //need to restrict to authorized admin

        pub system_program: Program<'info, System>,

        #[account(init,payer = admin,seeds=[b"escrow"],bump,space = BetAccount::LEN,)]
        pub vault_pda_account: Account<'info, BetVaultAccount>,
}


#[derive(Accounts)]
pub struct PlaceBet<'info> {
    #[account(mut)]
    pub bettor: Signer<'info>,

    pub bet: Account<'info, BetAccount>,

    pub system_program: Program<'info, System>,

    #[account( mut, constraint = bet.vault_pda == *vault_pda_account.to_account_info().key)]
    pub vault_pda_account: Account<'info, BetVaultAccount>,
}

// #[derive(Accounts)]
// pub struct DeclareResult<'info> {

// }

// #[derive(Accounts)]
// pub struct ClaimWinnings<'info> {

// }


// pub enum BetState {
//     Started,
//     Closed,
//     ResultOut,
// }

#[account]
pub struct BetAccount {
    pub bet_id: u32,
    pub bet_result: Option<u8>,
    pub bettor_list:Vec<Pubkey>,
    pub timestamp: i64,
    pub vault_pda : Pubkey,
    // pub betstate: BetState,
}

#[account]
pub struct BetVaultAccount {}

// const BET_STATE: usize = 8; //just to be safe
const BET_RESULT: usize = 8; //just to be safe
const BET_ID_LENGTH: usize = 8;
const DISCRIMINATOR_LENGTH: usize = 8;
const PUBLIC_KEY_LENGTH: usize = 32;
const TIMESTAMP_LENGTH: usize = 8;


impl BetAccount {
    const LEN: usize = DISCRIMINATOR_LENGTH
        + (PUBLIC_KEY_LENGTH * 50)// Betters.
        + TIMESTAMP_LENGTH // Timestamp.
        + BET_ID_LENGTH 
        // + BET_STATE
        + BET_RESULT;

}

#[error_code]
pub enum ErrorCode {
    #[msg("The bet limit for number of users is reached")]
    BetLimitFull,
    #[msg("Invalid Bet")]
    InvalidBet,
}



// use anchor_lang::{prelude::*, solana_program::system_program};

// declare_id!("E41ZWCPjxsHmAv6DhUdfduj8W2bt7VCnq4RiypAL1RYc");

// #[program]
// pub mod lock {
    // use anchor_lang::solana_program::{
    //     lamports,
    //     program::{invoke, invoke_signed},
    //     system_instruction::{transfer , assign_with_seed, assign}
    // };

//     use super::*;
//     pub fn initialize(ctx: Context<Initialize>, bump: u8, escrow_bump: u8, authority: Pubkey) -> Result<()> {
//         let lock_account = &mut ctx.accounts.lock_account;
//         //let tx  = &assign(lock_account.to_account_info().key, ctx.accounts.owner.to_account_info().key);
//         lock_account.authority = authority;
//         lock_account.owner = *ctx.accounts.owner.key;
//         lock_account.locked = true;
//         lock_account.bump = bump;
//         lock_account.escrow_bump = escrow_bump;
//         lock_account.escrow_pda = *ctx.accounts.lock_escrow_account.to_account_info().key;
//         Ok(())
//     }
//     pub fn unlock(ctx: Context<Unlock>) -> Result<()> {
//         let lock_account = &mut ctx.accounts.lock_account;
//         lock_account.locked = false;
//         Ok(())
//     }
//     pub fn lock(ctx: Context<Unlock>) -> Result<()> {
//         let lock_account = &mut ctx.accounts.lock_account;
//         lock_account.locked = true;
//         Ok(())
//     }
//     pub fn withdraw(ctx: Context<Withdraw>, lamports: u64) -> Result<()> {
//         let lock_account = &mut ctx.accounts.lock_account;
//         let lock_escrow_account = &mut ctx.accounts.lock_escrow_account;

//         **lock_escrow_account.to_account_info().try_borrow_mut_lamports()? -= lamports;
//         **ctx.accounts.owner.to_account_info().try_borrow_mut_lamports()? += lamports;
//         Ok(())
//     }

//     pub fn payin(ctx: Context<Payin>, lamports: u64) -> Result<()> {
//         let lock_account = &mut ctx.accounts.lock_account;
//         let lock_escrow_account = &mut ctx.accounts.lock_escrow_account;
//         let transfer_instruction = &transfer(
//             &lock_account.owner,
//             &lock_escrow_account.to_account_info().key,
//             lamports,
//         );
//         msg!("Paying in {}", lamports);
//         invoke(
//             transfer_instruction,
//             &[
//                 ctx.accounts.owner.to_account_info(),
//                 lock_escrow_account.to_account_info(),       
//             ]
//         );
//         Ok(())
//     }
// }

// #[derive(Accounts)]
// #[instruction(bump: u8, escrow_bump: u8)]
// pub struct Initialize<'info> {
    // #[account(init,
    // payer=owner,
    // space=8 + 32 + 32 + 1 + 1 + 1 + 32 ,
    // seeds=[owner.key().as_ref()],
    // bump)
    // ]
//     pub lock_account: Account<'info, LockAccount>,
//     #[account(mut)]
//     pub owner: Signer<'info>,
//     pub system_program: Program<'info, System>,
//     #[account(init,
//         payer=owner,
//         space=8,
//         seeds=[owner.key().as_ref(),b"escrow"],
//         bump)
//         ]
//     pub lock_escrow_account: Account<'info, LockEscrowAccount>,
// }

// #[derive(Accounts)]
// pub struct Unlock<'info> {
//     #[account(mut, has_one = authority)]
//     pub lock_account: Account<'info, LockAccount>,
//     #[account(signer)]
//     pub authority: AccountInfo<'info>,
// }

// #[derive(Accounts)]
// pub struct Withdraw<'info> {
//     #[account(mut,has_one=owner, constraint = !lock_account.locked, close=owner)]
//     pub lock_account: Account<'info, LockAccount>,
//     #[account(mut, signer)]
//     pub owner: AccountInfo<'info>,
//     pub system_program: Program<'info, System>,
//     #[account( mut, constraint = lock_account.escrow_pda == *lock_escrow_account.to_account_info().key)]
//     //#[account( mut)]
//     pub lock_escrow_account: Account<'info, LockEscrowAccount>
// }

// #[derive(Accounts)]
// pub struct Payin<'info> {
//     #[account(has_one = owner)]
//     pub lock_account: Account<'info, LockAccount>,
//     #[account(signer)]
//     pub owner: AccountInfo<'info>,
//     pub system_program: Program<'info, System>,
//     #[account( mut, constraint = lock_account.escrow_pda == *lock_escrow_account.to_account_info().key)]
//     pub lock_escrow_account: Account<'info, LockEscrowAccount>
// }


// #[account]
// pub struct LockAccount {
//     pub owner: Pubkey,
//     pub authority: Pubkey,
//     pub locked: bool,
//     pub bump: u8,
//     pub escrow_bump: u8,
//     pub escrow_pda: Pubkey
// }


// #[account]
// pub struct LockEscrowAccount {}