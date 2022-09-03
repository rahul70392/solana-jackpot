import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import { SolanaJackpot } from "../target/types/solana_jackpot";
const numberToBN = require('number-to-bn');
const { PublicKey } = anchor.web3;
import * as assert from "assert";


anchor.setProvider(anchor.AnchorProvider.env());
const program = anchor.workspace.SolanaJackpot as Program<SolanaJackpot>;

//First test - initialize
it('admin can initialize a new bet', async () => {

    let betId  = numberToBN('1');
    //every bet is identified as unique, only by its betPda account. So unique seeds need to be sent for each bet.
    //Like "seed1" will generate a unique pda, then "seed2", "seed3" and so on.
    //similarly vaultpda is also unique for each bet, and needs to change with every bet.
    const [betPda, betBump] = await PublicKey.findProgramAddress([Buffer.from('seed')], program.programId);
    const [vaultPda, vaultBump] = await PublicKey.findProgramAddress([Buffer.from('escrow')], program.programId);

    const vaultInitialBal = await program.provider.connection.getBalance(vaultPda);
    console.log("current bet vault initial balance in first test-------->",vaultInitialBal);

    const _admin = program.provider.publicKey;

        await program.rpc.initializeBet(betId, {
            accounts: {
                bet: betPda,
                admin: _admin,
                systemProgram: anchor.web3.SystemProgram.programId,
                vaultPdaAccount: vaultPda,
            },
            signers: [],
        });

        const vaultfinalBal = await program.provider.connection.getBalance(vaultPda);
        console.log("current bet vault final balance after first test-------->",vaultfinalBal);
    });



//second test - place bet
it('can place a new bet', async () => {

    // let betId  = numberToBN('1');
    let betAmount = numberToBN('2000000000');
    let betPosition = numberToBN('1');
    const [betPda, betBump] = await PublicKey.findProgramAddress([Buffer.from('seed')], program.programId);
    const [vaultPda, vaultBump] = await PublicKey.findProgramAddress([Buffer.from('escrow')], program.programId);
    // const [bettorPendingWinAmountPda, bettorPendingWinAmountBump] = await PublicKey.findProgramAddress([program.provider.publicKey.toBuffer(),Buffer.from('bettor')], program.programId);
    // const [bettorCurrentBetDetailsPda, bettorCurrentBetDetailsBump] = await PublicKey.findProgramAddress([program.provider.publicKey.toBuffer(),Buffer.from('details')], program.programId);

    const vaultInitialBal = await program.provider.connection.getBalance(vaultPda);
    console.log("current bet vault initial balance in 2nd test-------->",vaultInitialBal);

        await program.rpc.placeBet( betAmount, betPosition, {
            accounts: {
                bettor: program.provider.publicKey,
                bet: betPda,
                systemProgram: anchor.web3.SystemProgram.programId,
                vaultPdaAccount: vaultPda,
            },
            signers: [],
        });
        
        const vaultFinalBal = await program.provider.connection.getBalance(vaultPda);
        assert.equal((vaultFinalBal-vaultInitialBal), betAmount);
    });


    //third test - place bet from 3 different users
it('can place a bets from 3 different users, different bet amounts and bet positions', async () => {


    // let betId  = numberToBN('1');
    let betAmount1 = numberToBN('1000000000');
    let betPosition1 = numberToBN('1');
    const [betPda, betBump] = await PublicKey.findProgramAddress([Buffer.from('seed')], program.programId);
    const [vaultPda, vaultBump] = await PublicKey.findProgramAddress([Buffer.from('escrow')], program.programId);
    // const [bettorPendingWinAmountPda, bettorPendingWinAmountBump] = await PublicKey.findProgramAddress([program.provider.publicKey.toBuffer(),Buffer.from('bettor')], program.programId);
    // const [bettorCurrentBetDetailsPda, bettorCurrentBetDetailsBump] = await PublicKey.findProgramAddress([program.provider.publicKey.toBuffer(),Buffer.from('details')], program.programId);

    const vaultInitialBal = await program.provider.connection.getBalance(vaultPda);
    console.log("current bet vault initial balance in 3rd test-------->",vaultInitialBal);

        await program.rpc.placeBet( betAmount1, betPosition1, {
            accounts: {
                bettor: program.provider.publicKey,
                bet: betPda,
                systemProgram: anchor.web3.SystemProgram.programId,
                vaultPdaAccount: vaultPda,
            },
            signers: [],
        });
        
        const User2 = anchor.web3.Keypair.generate();
        const signature = await program.provider.connection.requestAirdrop(User2.publicKey, 10000000000);
        await program.provider.connection.confirmTransaction(signature);

        const User3 = anchor.web3.Keypair.generate();
        const signature2 = await program.provider.connection.requestAirdrop(User3.publicKey, 10000000000);
        await program.provider.connection.confirmTransaction(signature2);

        let betAmount2 = numberToBN('2000000000');
        let betPosition2 = numberToBN('2');

        let betAmount3 = numberToBN('3000000000');
        let betPosition3 = numberToBN('3');

        await program.rpc.placeBet( betAmount2, betPosition2, {
            accounts: {
                bettor: User2.publicKey,
                bet: betPda,
                systemProgram: anchor.web3.SystemProgram.programId,
                vaultPdaAccount: vaultPda,
            },
            signers: [User2],
        });

        await program.rpc.placeBet( betAmount3, betPosition3, {
            accounts: {
                bettor: User3.publicKey,
                bet: betPda,
                systemProgram: anchor.web3.SystemProgram.programId,
                vaultPdaAccount: vaultPda,
            },
            signers: [User3],
        });



        const vaultFinalBal = await program.provider.connection.getBalance(vaultPda);
        console.log("vault final balance after 3rd test-------->",vaultFinalBal);
    });

    //fourth test - declare result 
it('admin can declare result', async () => {

    // let betId  = numberToBN('1');
    // let betAmount = numberToBN('2000000000');
    // let betPosition = numberToBN('1');
    const [betPda, betBump] = await PublicKey.findProgramAddress([Buffer.from('seed')], program.programId);



        await program.rpc.declareResult({
            accounts: {
                admin: program.provider.publicKey,
                bet: betPda,
                systemProgram: anchor.web3.SystemProgram.programId,
            },
            signers: [],
        });

        const betAccount = await program.account.betAccount.fetch(betPda);
        console.log("BetAccountDEtails after 4th test-------------",betAccount);

    });

    //second test - place bet
it('can claim rewards', async () => {

    const [betPda, betBump] = await PublicKey.findProgramAddress([Buffer.from('seed')], program.programId);
    const [vaultPda, vaultBump] = await PublicKey.findProgramAddress([Buffer.from('escrow')], program.programId);
    // const [bettorPendingWinAmountPda, bettorPendingWinAmountBump] = await PublicKey.findProgramAddress([program.provider.publicKey.toBuffer(),Buffer.from('bettor')], program.programId);
    // const [bettorCurrentBetDetailsPda, bettorCurrentBetDetailsBump] = await PublicKey.findProgramAddress([program.provider.publicKey.toBuffer(),Buffer.from('details')], program.programId);

    const vaultInitialBal = await program.provider.connection.getBalance(vaultPda);
    console.log("current bet vault initial balance in 2nd test-------->",vaultInitialBal);

        await program.rpc.claimRewards({
            accounts: {
                bettor: program.provider.publicKey,
                bet: betPda,
                systemProgram: anchor.web3.SystemProgram.programId,
                vaultPdaAccount: vaultPda,
            },
            signers: [],
        });
        
        const vaultFinalBal = await program.provider.connection.getBalance(vaultPda);
        // assert.equal((vaultFinalBal-vaultInitialBal), betAmount);
        console.log("vault final balance after 5th test i.e. claim rewards",vaultFinalBal);
    });