import { Connection, Keypair, LAMPORTS_PER_SOL } from "@solana/web3.js";

export async function createAndFundAccount(connection: Connection) {
  const newAccount = Keypair.generate();
  console.log("New Account Public Key:", newAccount.publicKey.toBase58());

  // Request an airdrop on Devnet
  const airdropSignature = await connection.requestAirdrop(
    newAccount.publicKey,
    1 * LAMPORTS_PER_SOL // Airdrop 1 SOL
  );

  // Confirm the airdrop transaction
  await connection.confirmTransaction(airdropSignature);
  console.log("Airdrop confirmed!");

  const balance = await connection.getBalance(newAccount.publicKey);
  console.log(`New account balance: ${balance / LAMPORTS_PER_SOL} SOL`);

  return newAccount;
}
