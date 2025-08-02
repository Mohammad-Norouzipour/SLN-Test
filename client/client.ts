import { PublicKey, Keypair } from "@solana/web3.js";
import { AccountClass } from "./libs/accountClass";
import * as callClass from "./libs/callClass.ts";
import * as eventClass from "./libs/eventClass.ts";

// const connection = new web3.Connection(
//   "https://api.devnet.solana.com	",
//   "confirmed"
// );
const acManager = new AccountClass("AccountManager");

const connection = pg.connection;

const account = await acManager.accountCreateAndFund(connection);

const pda = await acManager.accountInitPDA(connection);

console.log(`PDA: ${pda.address}`);
console.log(`Bump: ${pda.bump}`);
console.log("Transaction submitted with hash: ", pda.tx_hash);
