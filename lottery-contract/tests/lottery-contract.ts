import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { LotteryContract } from "../target/types/lottery_contract";
import { assert } from "chai";

const MASTER_SEED = Buffer.from("master");
const LOTTERY_SEED = Buffer.from("lottery");
const TICKET_SEED = Buffer.from("ticket");

let masterPda: anchor.web3.PublicKey;
let lotteryPda: anchor.web3.PublicKey;

const provider = anchor.AnchorProvider.env();
anchor.setProvider(provider);

const program = anchor.workspace.LotteryContract as Program<LotteryContract>;

const payer = provider.wallet;

it("Initializes master account", async () => {
  [masterPda] = anchor.web3.PublicKey.findProgramAddressSync(
    [MASTER_SEED],
    program.programId
  );

  await program.methods
    .initMaster()
    .accountsStrict({
      master: masterPda,
      payer: payer.publicKey,
      systemProgram: anchor.web3.SystemProgram.programId,
    })
    .rpc();

  const master = await program.account.master.fetch(masterPda);
  assert.equal(master.lastId, 0);
});

it("Creates a lottery", async () => {
  const ticketPrice = new anchor.BN(1_000_000); // 0.001 SOL
  const maxTickets = 5;

  // master.last_id + 1 = 1
  [lotteryPda] = anchor.web3.PublicKey.findProgramAddressSync(
    [LOTTERY_SEED, Buffer.from([1])],
    program.programId
  );

  await program.methods
    .createLottery(ticketPrice, maxTickets)
    .accountsStrict({
      lottery: lotteryPda,
      master: masterPda,
      authority: payer.publicKey,
      systemProgram: anchor.web3.SystemProgram.programId,
    })
    .rpc();

  const lottery = await program.account.lottery.fetch(lotteryPda);

  assert.equal(lottery.ticketPrice.toNumber(), ticketPrice.toNumber());
  assert.equal(lottery.maxTickets, maxTickets);
  assert.equal(lottery.ticketsSold, 0);
  assert.isTrue(lottery.isActive);
});

it("Buys a ticket", async () => {
  const ticketId = 1;

  const [ticketPda] = anchor.web3.PublicKey.findProgramAddressSync(
    [TICKET_SEED, lotteryPda.toBuffer(), Buffer.from([ticketId])],
    program.programId
  );

  await program.methods
    .buyTicket()
    .accountsStrict({
      lottery: lotteryPda,
      ticket: ticketPda,
      buyer: payer.publicKey,
      systemProgram: anchor.web3.SystemProgram.programId,
    })
    .rpc();

  const lottery = await program.account.lottery.fetch(lotteryPda);
  const ticket = await program.account.ticket.fetch(ticketPda);

  assert.equal(lottery.ticketsSold, 1);
  assert.equal(ticket.id, 1);
  assert.ok(ticket.buyer.equals(payer.publicKey));
});

it("Draws a winner", async () => {
  await program.methods
    .drawWinner()
    .accountsStrict({
      lottery: lotteryPda,
      authority: payer.publicKey,
    })
    .rpc();

  const lottery = await program.account.lottery.fetch(lotteryPda);

  assert.isNotNull(lottery.winnerId);
  assert.equal(lottery.winnerId!, 1);
});
