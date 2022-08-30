use anchor_lang::prelude::*;
// use anchor_lang::solana_program::system_program;

declare_id!("DP1BGMQdhhTE6CehdvhgQTBZe8mtA4RvmcAg8DQ6oxkd");


#[program]
pub mod solana_jackpot {
    use super::*;
    const VAULT_PDA_SEED: &[u8] = b"bet";
    pub fn initialize_bet(ctx: Context<InitializeBet>, betid: u32) -> Result<()> {
        let betaccount: &mut Account<BetAccount> = &mut ctx.accounts.bet;

        let clock: Clock = Clock::get().unwrap();
        let bet_id : u32 = betid;
        let mut vec = Vec::new();
        
        betaccount.betid = bet_id;
        betaccount.betresult = None;
        betaccount.bettorlist = vec.clone();
        betaccount.timestamp = clock.unix_timestamp;
        // betaccount.betstate = BetState::Started;
        let (pda, _bump_seed) = Pubkey::find_program_address(&[VAULT_PDA_SEED], ctx.program_id);
        betaccount.vaultpda = pda;
        Ok(())
    }
}


#[derive(Accounts)]
pub struct InitializeBet<'info> {
        #[account(init,payer = admin,space = BetAccount::LEN,)]
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
    pub betid: u32,
    pub betresult: Option<u8>,
    pub bettorlist:Vec<Pubkey>,
    pub timestamp: i64,
    pub vaultpda : Pubkey,
    // pub betstate: BetState,
}

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