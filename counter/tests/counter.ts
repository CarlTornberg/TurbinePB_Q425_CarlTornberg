import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Counter } from "../target/types/counter";
import { Connection, Keypair, LAMPORTS_PER_SOL, PublicKey, Signer, SOLANA_SCHEMA, Transaction, TransactionConfirmationStrategy } from "@solana/web3.js";
import { expect, should } from "chai";
import assert from "assert";

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

  it("Cannot initialize count account twice", async () => {
    try {
      await program.methods
      .initializeAccount()
      .accounts({signer: bob.publicKey})
      .signers([bob])
      .rpc({commitment: "confirmed"});
      should().fail("Should not be able to initialize account twice.");
    }
    catch(error) {}
  });

  it("Increment once bob", async () => {
    const countPDA = PublicKey.findProgramAddressSync(
      [anchor.utils.bytes.utf8.encode("count"),
        bob.publicKey.toBuffer()], 
      program.programId);
    const tx = await program.methods
      .incrementAccountCount()
      .accounts({signer: bob.publicKey})
      .signers([bob])
      .rpc({commitment: "confirmed"});

    const countAccount = await program.account.countAccount.fetch(countPDA[0], "confirmed");
    should().equal(countAccount.count.toNumber() , 1);
  });

  it("Decrement once bob", async () => {
    const countPDA = PublicKey.findProgramAddressSync(
      [anchor.utils.bytes.utf8.encode("count"),
        bob.publicKey.toBuffer()], 
      program.programId);
    const tx = await program.methods
      .devrementAccountCount()
      .accounts({signer: bob.publicKey})
      .signers([bob])
      .rpc({commitment: "confirmed"});

    const countAccount = await program.account.countAccount.fetch(countPDA[0], "confirmed");
    should().equal(countAccount.count.toNumber() , 0);
  });

  it("Should not decrement bob. Underflow", async () => {
  
    try {
      await program.methods
      .devrementAccountCount()
      .accounts({signer: bob.publicKey})
      .signers([bob])
      .rpc({commitment: "confirmed"});
      should().fail("Should not be able to decrement.");
    }
    catch(error) {
      assert.strictEqual(error.error.errorCode.code, "CountOverflowUnderflow");
    }
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

  function getPDA() {
    PublicKey.findProgramAddressSync(
      [anchor.utils.bytes.utf8.encode("count"),
        bob.publicKey.toBuffer()], 
      program.programId);
  }
});
