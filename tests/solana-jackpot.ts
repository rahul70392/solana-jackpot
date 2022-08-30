import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import { SolanaJackpot } from "../target/types/solana_jackpot";
const numberToBN = require('number-to-bn');
 


anchor.setProvider(anchor.AnchorProvider.env());
const program = anchor.workspace.SolanaJackpot as Program<SolanaJackpot>;

//First test
it('can initialize a new bet', async () => {
    const bet = anchor.web3.Keypair.generate();
    let betId  = numberToBN('1');

        await program.rpc.initializeBet(betId, {
            accounts: {
                bet: bet.publicKey,
                admin: program.provider.wallet.publicKey,
                systemProgram: anchor.web3.SystemProgram.programId,
            },
            signers: [bet],
        });
        const betAccount = await program.account.betAccount.fetch(bet.publicKey);
        console.log("BET pda account details----------------------",betAccount.vaultpda.toBase58());
    });
