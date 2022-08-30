use anchor_lang::prelude::*;
// use anchor_lang::solana_program::system_program;

declare_id!("6wphNBvMTUwuwgunq79J2o7ihJVdcXmDwzskN66x9GP3");

#[program]
pub mod solana_jackpot {
    use super::*;
    pub fn initialize_bet(ctx: Context<InitializeBet>, betid: u64, bump_value: u8) -> Result<()> {
        let betaccount: &mut Account<BetAccount> = &mut ctx.accounts.bet;

        let clock: Clock = Clock::get().unwrap();
        let betId : u64 = betid;
        let mut vec = Vec::new();
        
        betaccount.betid = betId;
        betaccount.betresult = None;
        betaccount.bettorlist = vec.clone();
        betaccount.timestamp = clock.unix_timestamp;
        // betaccount.betstate = BetState::Started;

        Ok(())
    }
}



#[derive(Accounts)]
#[instruction(betid: u64, bump_value: u8)]
pub struct InitializeBet<'info> {
        // Derived PDA
        #[account(
            init,
            payer = admin,
            seeds=[b"bet".as_ref(), admin.key().as_ref(), betid.to_le_bytes().as_ref()],
            bump,
            space = BetAccount::LEN,
        )]
        bet: Account<'info, BetAccount>,
        #[account(mut)]
        admin: Signer<'info>, //need to restrict to authorized admin
        system_program: Program<'info, System>,
}


// #[derive(Accounts)]
// pub struct PlaceBet<'info> {

// }

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
    pub betid: u64,
    pub betresult: Option<u8>,
    pub bettorlist:Vec<Pubkey>,
    pub timestamp: i64,
    // pub betstate: BetState,
}

const BET_STATE: usize = 8; //just to be safe
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
        + BET_STATE
        + BET_RESULT;

        
}

#[error_code]
pub enum ErrorCode {
    #[msg("The bet limit for number of users is reached")]
    BetLimitFull,
    #[msg("Invalid Bet")]
    InvalidBet,
}