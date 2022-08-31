import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import { SolanaJackpot } from "../target/types/solana_jackpot";
const numberToBN = require('number-to-bn');
const { PublicKey } = anchor.web3;
import * as assert from "assert";


anchor.setProvider(anchor.AnchorProvider.env());
const program = anchor.workspace.SolanaJackpot as Program<SolanaJackpot>;

//First test - initialize
it('can initialize a new bet', async () => {

    let betId  = numberToBN('1');
    const [betPda, betBump] = await PublicKey.findProgramAddress([Buffer.from('seed')], program.programId);
    const [vaultPda, vaultBump] = await PublicKey.findProgramAddress([Buffer.from('escrow')], program.programId);
    console.log("betPda---------------------------",program.provider.publicKey);

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
    });



//second test - place bet
it('can place a new bet', async () => {

    let betId  = numberToBN('1');
    let betAmount = numberToBN('2000000000');
    const [betPda, betBump] = await PublicKey.findProgramAddress([Buffer.from('seed')], program.programId);
    const [vaultPda, vaultBump] = await PublicKey.findProgramAddress([Buffer.from('escrow')], program.programId);

    const vaultInitialBal = await program.provider.connection.getBalance(vaultPda);

        await program.rpc.placeBet(betId, betAmount, {
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