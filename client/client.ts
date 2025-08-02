import { PublicKey , Keypair} from "@solana/web3.js";
import {createAndFundAccount} from "./libs/accountClass";

const connection = new web3.Connection(
  "https://api.devnet.solana.com	",
  "confirmed"
);

const account = await createAndFundAccount(connection);

const pdaInit = new web3.Transaction();
const programAddress = new PublicKey(pg.PROGRAM_ID);

const seeds = [, sender.toBuffer()];
const [pda, bump] = await PublicKey.findProgramAddressSync(
  seeds,
  programAddress
);

pdaInit.add(
  new web3.TransactionInstruction({
    keys: [],
    programId: new web3.PublicKey(pg.PROGRAM_ID),
  })
);

console.log("Forwarding Transaction");

const tx_hash = await web3.sendAndConfirmTransaction(pg.connection, pdaInit, [
  pg.wallet.keypair,
]);

console.log("sender address is " + sender.toBase58());
console.log("base balance is ", JSON.stringify(basebalance, null, 2));
console.log("new balance is ", JSON.stringify(newbalance, null, 2));

console.log(`PDA: ${pda}`);
console.log(`Bump: ${bump}`);
console.log("Transaction submitted with hash: ", tx_hash);
