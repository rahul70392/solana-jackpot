use anchor_lang::{prelude::*};
declare_id!("DP1BGMQdhhTE6CehdvhgQTBZe8mtA4RvmcAg8DQ6oxkd");


#[program]
pub mod solana_jackpot {
    use super::*;

        use anchor_lang::solana_program::{
        program::{invoke},
        system_instruction::{transfer},
    };

    pub fn initialize_bet(ctx: Context<InitializeBet>, betid: u64) -> Result<()> {
        let bet_account: &mut Account<BetAccount> = &mut ctx.accounts.bet;

        let clock: Clock = Clock::get().unwrap();
        let bet_id : u64 = betid;
        let vec1 = Vec::new();
        let vec2 = Vec::new();
        let vec3 = Vec::new();

        bet_account.bet_id = bet_id;
        bet_account.bet_result = None;
        bet_account.bettor_list = vec1.clone();
        bet_account.bettor_list_amount = vec2.clone();
        bet_account.bettor_guess = vec3.clone();
        bet_account.timestamp = clock.unix_timestamp;
        bet_account.vault_pda = *ctx.accounts.vault_pda_account.to_account_info().key;
        bet_account.total_bet_amount = 0;
        bet_account.active_status = true;
        bet_account.total_amount_bet_on_correct = 0;
        Ok(())
    }

    pub fn place_bet(ctx: Context<PlaceBet>, bet_amount: u64, bet_position:u8) -> Result<()> {
        let bet_account: &mut Account<BetAccount> = &mut ctx.accounts.bet;
        let vault_pda_account: &mut Account<BetVaultAccount> = &mut ctx.accounts.vault_pda_account;

        let transfer_instruction = &transfer(
                        &ctx.accounts.bettor.key,
                        &vault_pda_account.to_account_info().key,
                        bet_amount,
                    );
                    msg!("betting with {} lamports", bet_amount);
                    invoke(
                        transfer_instruction,
                        &[
                            ctx.accounts.bettor.to_account_info(),
                            vault_pda_account.to_account_info(),       
                        ]
                    )?;

        bet_account.bettor_list.push(*ctx.accounts.bettor.key);
        bet_account.bettor_list_amount.push(bet_amount);
        bet_account.bettor_guess.push(Some(bet_position));
        bet_account.total_bet_amount = bet_account.total_bet_amount + bet_amount;
 
        Ok(())
    }

    pub fn declare_result(ctx: Context<DeclareResult>) -> Result<()> {
        let bet_account: &mut Account<BetAccount> = &mut ctx.accounts.bet;


        //TBD-//Need to generate result randomly in a provably fair way and then distribute the prize
        bet_account.bet_result = Some(1);
        let res_temp:Option<u8> = Some(1);
        let mut total_amount_bet = 0;

        //iterare to calculate the total amount bet on the correct result, i.e. total bet by the winners
        for (i, x) in bet_account.bettor_guess.iter().enumerate() {
                if *x == res_temp
                {
                    total_amount_bet =  total_amount_bet + bet_account.bettor_list_amount[i];
                }
            }

        bet_account.total_amount_bet_on_correct = total_amount_bet;
        
        Ok(())
    }

    pub fn claim_rewards(ctx: Context<ClaimRewards>) -> Result<()> {
        let bet_account: &mut Account<BetAccount> = &mut ctx.accounts.bet;
        let vault_pda_account: &mut Account<BetVaultAccount> = &mut ctx.accounts.vault_pda_account;
        let key_bettor = *ctx.accounts.bettor.key;

        //iterare to calculate the total amount bet on the correct result, i.e. total bet by the winners
        for (i, x) in bet_account.bettor_list.iter().enumerate() {
            if *x == key_bettor
            {
                if bet_account.bettor_guess[i] == bet_account.bet_result
                    {
                        let reward_amount : u64 = (bet_account.bettor_list_amount[i] * bet_account.total_bet_amount)/ bet_account.total_amount_bet_on_correct ;

                        if **(&vault_pda_account.to_account_info()).try_borrow_lamports()? < reward_amount {
                            // return Err();
                        }
                        **(&vault_pda_account.to_account_info()).try_borrow_mut_lamports()? -= reward_amount;
                        **ctx.accounts.bettor.try_borrow_mut_lamports()? += reward_amount;
                }
            }
        }
        
        Ok(())
    }
}

//TBD onlyAdmin can call- need to restrict
#[derive(Accounts)]
pub struct InitializeBet<'info> {
        #[account(init,payer = admin,seeds=[b"seed3"],bump,space = 1500,)]
        pub bet: Account<'info, BetAccount>,

        #[account(mut)]
        pub admin: Signer<'info>, //need to restrict to authorized admin

        pub system_program: Program<'info, System>,

        #[account(init,payer = admin,seeds=[b"escrow3"],bump,space = 1500,)]
        pub vault_pda_account: Account<'info, BetVaultAccount>,
}

//public function
#[derive(Accounts)]
pub struct PlaceBet<'info> {
    #[account(mut)]
    pub bettor: Signer<'info>,

    #[account(mut)]
    pub bet: Account<'info, BetAccount>,

    pub system_program: Program<'info, System>,

    #[account(mut, constraint = bet.vault_pda == *vault_pda_account.to_account_info().key)]
    pub vault_pda_account: Account<'info, BetVaultAccount>,

}


//TBD onlyAdmin can call- need to restrict
#[derive(Accounts)]
pub struct DeclareResult<'info> {
    // #[account(mut)]
    pub admin: Signer<'info>,

    #[account(mut)]
    pub bet: Account<'info, BetAccount>,

    pub system_program: Program<'info, System>,

}

//public function
#[derive(Accounts)]
    pub struct ClaimRewards<'info> {
    #[account(mut)]
    pub bettor: Signer<'info>,

    #[account(mut)]
    pub bet: Account<'info, BetAccount>,

    pub system_program: Program<'info, System>,

    #[account(mut, constraint = bet.vault_pda == *vault_pda_account.to_account_info().key)]
    pub vault_pda_account: Account<'info, BetVaultAccount>,
}


//Stores all the current bet details
#[account]
pub struct BetAccount {
    pub bet_id: u64,
    pub bet_result: Option<u8>, //Some(1) or Some(2) or Some(3). based on the position
    pub bettor_guess : Vec<Option<u8>>,
    pub bettor_list:Vec<Pubkey>,
    pub bettor_list_amount:Vec<u64>,
    pub timestamp: i64,
    pub vault_pda : Pubkey,
    pub total_bet_amount : u64,
    pub total_amount_bet_on_correct :u64,
    pub active_status : bool,
}

//stores all the current bet money
#[account]
pub struct BetVaultAccount {}



// const BET_RESULT: usize = 8; //just to be safe
// const BET_ID_LENGTH: usize = 8;
// const DISCRIMINATOR_LENGTH: usize = 8;
// const PUBLIC_KEY_LENGTH: usize = 32;
// const TIMESTAMP_LENGTH: usize = 8;


// impl BetAccount {
//     const LEN: usize = DISCRIMINATOR_LENGTH
//         + (PUBLIC_KEY_LENGTH * 100)// Betters.
//         + TIMESTAMP_LENGTH // Timestamp.
//         + BET_ID_LENGTH 
//         + BET_RESULT;
// }

#[error_code]
pub enum ErrorCode {
    #[msg("The bet limit for number of users is reached")]
    BetLimitFull,
    #[msg("Invalid Bet")]
    InvalidBet,
}
