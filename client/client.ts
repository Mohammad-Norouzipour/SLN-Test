import { PublicKey } from "@solana/web3.js";
const transaction = new web3.Transaction();

const programAddress = new PublicKey(pg.PROGRAM_ID);

transaction.add(
  new web3.TransactionInstruction({
    keys: [],
    programId: new web3.PublicKey(pg.PROGRAM_ID),
  })
);

console.log("Forwarding Transaction");

const tx_hash = await web3.sendAndConfirmTransaction(
  pg.connection,
  transaction,
  [pg.wallet.keypair]
);

const seeds = [Buffer.from("helloWorldmyluckyfriendbaby!")];
const [pda, bump] = await PublicKey.findProgramAddressSync(
  seeds,
  programAddress
);

console.log(`PDA: ${pda}`);
console.log(`Bump: ${bump}`);
console.log("Transaction submitted with hash: ", tx_hash);
