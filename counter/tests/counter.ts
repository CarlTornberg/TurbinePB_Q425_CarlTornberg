import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Counter } from "../target/types/counter";
import { Connection, Keypair, LAMPORTS_PER_SOL, PublicKey, Signer, SOLANA_SCHEMA, Transaction, TransactionConfirmationStrategy } from "@solana/web3.js";
import { expect, should } from "chai";
import { BN } from "bn.js";

describe("counter", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());
  const provider = anchor.AnchorProvider.env();
  const conn = provider.connection;
  const program = anchor.workspace.counter as Program<Counter>;

  const alice = anchor.web3.Keypair.generate();
  const bob = anchor.web3.Keypair.generate();

  it("Initialize count account bob", async () => {
    await airdrop(bob.publicKey, LAMPORTS_PER_SOL);

    const tx = await program.methods
      .initializeAccount()
      .accounts({
        signer: bob.publicKey, 
        })
      .signers([bob])
      .rpc({commitment: "confirmed"});
    console.log("Your transaction signature", tx);
  });

  it("Count once bob", async () => {
    const countPDA = PublicKey.findProgramAddressSync(
      [anchor.utils.bytes.utf8.encode("count"),
        bob.publicKey.toBuffer()], 
      program.programId);
    const tx = await program.methods
      .count()
      .accounts({signer: bob.publicKey})
      .signers([bob])
      .rpc({commitment: "confirmed"});

    const countAccount = await program.account.countAccount.fetch(countPDA[0], "confirmed");
    should().equal(countAccount.count.toNumber() , 1);
  });

  async function airdrop(addr: PublicKey, amount: number) {
    const latestBlockhash =  await conn.getLatestBlockhash();
    return await conn.confirmTransaction(
      {
        signature: await conn.requestAirdrop(addr, amount),
        blockhash: latestBlockhash.blockhash,
        lastValidBlockHeight: latestBlockhash.lastValidBlockHeight,
      }, "confirmed");
  }
});
