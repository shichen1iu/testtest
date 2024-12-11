import * as anchor from "@coral-xyz/anchor";
import { BN, Program } from "@coral-xyz/anchor";
import { Testtest } from "../target/types/testtest";
import {
  TOKEN_PROGRAM_ID,
  ASSOCIATED_TOKEN_PROGRAM_ID,
  createMint,
  createAssociatedTokenAccount,
  mintTo,
  getAssociatedTokenAddress,
  getAccount,
} from "@solana/spl-token";

describe("testtest", () => {
  // Configure the client to use the local cluster.
  let provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);
  const user = provider.wallet as anchor.Wallet;

  const program = anchor.workspace.Testtest as Program<Testtest>;

  let usdcMint: anchor.web3.PublicKey;
  let userUsdcAccount: anchor.web3.PublicKey;
  before(async () => {
    usdcMint = await createMint(
      provider.connection,
      user.payer,
      user.publicKey,
      null,
      6
    );

    userUsdcAccount = await createAssociatedTokenAccount(
      provider.connection,
      user.payer,
      usdcMint,
      user.publicKey
    );

    // Mint some USDC to the user
    await mintTo(
      provider.connection,
      user.payer,
      usdcMint,
      userUsdcAccount,
      user.payer,
      10_000_000_000
    );
  });
  it("pay sol", async () => {
    const payerCheck = new anchor.web3.PublicKey(
      "9qMknujRc8eqBZ6gSrypjYyzNNpwiwASxocKdAfg563C"
    );
    const receiver = new anchor.web3.PublicKey(
      "5KcXQcB2mJXehp86Gmg2ydWsoEKvDt7M9Y8bdHCnbmzY"
    );
    // Add your test here.
    const tx = await program.methods
      .paySol(new BN(100_000_000), payerCheck)
      .accounts({
        payer: user.publicKey,
        receiver: receiver,
      })
      .rpc();
    console.log("Your transaction signature", tx);
  });

  it("pay spl", async () => {
    const receiver = new anchor.web3.PublicKey(
      "5KcXQcB2mJXehp86Gmg2ydWsoEKvDt7M9Y8bdHCnbmzY"
    );
    const tx = await program.methods
      .paySpl(new BN(100_000_000))
      .accounts({
        payer: user.publicKey,
        payerTokenAccount: userUsdcAccount,
        receiver: receiver,
        mint: usdcMint,
        tokenProgram: TOKEN_PROGRAM_ID,
      })
      .rpc();
    console.log("pay spl tx", tx);
  });
});
