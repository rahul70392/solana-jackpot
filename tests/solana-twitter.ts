import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import { SolanaTwitter } from "../target/types/solana_twitter";
import * as assert from "assert";

it('can send a new tweet', async () => {
anchor.setProvider(anchor.AnchorProvider.env());
const program = anchor.workspace.SolanaTwitter as Program<SolanaTwitter>;

// Call the "SendTweet" instruction.
  const tweet = anchor.web3.Keypair.generate();
  await program.rpc.sendTweet('veganism', 'Hummus, am I right?', {
      accounts: {
          tweet: tweet.publicKey,
          author: program.provider.wallet.publicKey,
          systemProgram: anchor.web3.SystemProgram.programId,
      },
      signers: [tweet],
  });

  // Fetch the account details of the created tweet.
  const tweetAccount = await program.account.tweet.fetch(tweet.publicKey);

    assert.equal(tweetAccount.author.toBase58(), program.provider.wallet.publicKey.toBase58());
    assert.equal(tweetAccount.topic, 'veganism');
    assert.equal(tweetAccount.content, 'Hummus, am I right?');
    assert.ok(tweetAccount.timestamp);
});