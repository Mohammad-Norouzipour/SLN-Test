import {
  Connection,
  Keypair,
  PublicKey,
  LAMPORTS_PER_SOL,
  SystemProgram,
} from "@solana/web3.js";

export class AccountClass {
  constructor(public name: String) {}

  public async accountCreateAndFund(connection: Connection): Promise<Keypair> {
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

  public async accountInitPDA(
    connection: Connection,
    caller: PublicKey,
    callee: PublicKey
  ): Promise<number> {
    const payer = caller;
    const trans = new web3.Transaction();

    const programAddress = new PublicKey(pg.PROGRAM_ID);

    const seeds = [caller.toBuffer(), callee.toBuffer()];

    const [pda, bump] = await PublicKey.findProgramAddressSync(
      seeds,
      programAddress
    );

    const accountSpace = 8 + 2 * 8000;

    // Get minimum rent exemption balance
    const lamports = await connection.getMinimumBalanceForRentExemption(
      accountSpace
    );

    const createAccountInstruction = SystemProgram.createAccount({
      fromPubkey: payer,
      newAccountPubkey: pda,
      lamports,
      space: accountSpace,
      programId: programAddress,
    });

    trans.add(createAccountInstruction);

    // Sign and send the transaction
    // await sendAndConfirmTransaction(connection, transaction, [payer]);

    console.log("Forwarding Transaction");

    const tx_hash = await web3.sendAndConfirmTransaction(pg.connection, trans, [
      pg.wallet.keypair,
    ]);

    return 0;
  }

  public async accountFlushPDA(connection: Connection): Promise<number> {
    return 0;
  }

  public async accountSaveSettings(): Promise<number> {
    return 0;
  }

  public async accountLoadSettings(): Promise<number> {
    return 0;
  }

  public async accountDeleteSettings(): Promise<number> {
    return 0;
  }
}
