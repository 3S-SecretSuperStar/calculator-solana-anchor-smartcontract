import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Calculator } from "../target/types/calculator";
import * as assert from "assert";

describe("calculator", () => {
  let provider = anchor.AnchorProvider.local();
  anchor.setProvider(provider);

  const calculator = anchor.web3.Keypair.generate();

  const program = anchor.workspace.Calculator as Program<Calculator>;

  it("Creates a calculator", async () => {
    await program.methods
      .create("Welcome to Solana")
      .accounts({
        calculator: calculator.publicKey,
        user: provider.wallet.publicKey,
      })
      .signers([calculator])
      .rpc();

    const account = await program.account.calculator.fetch(
      calculator.publicKey
    );
    assert.ok(account.greeting === "Welcome to Solana");
  });

  it("Adds two numbers", async () => {
    await program.methods
      .add(new anchor.BN(2), new anchor.BN(3))
      .accounts({
        calculator: calculator.publicKey,
      })
      .rpc();
    const account = await program.account.calculator.fetch(
      calculator.publicKey
    );
    assert.ok(account.result.eq(new anchor.BN(5)));
  });

  it("Substraction two numbers", async () => {
    await program.methods
      .sub(new anchor.BN(5), new anchor.BN(1))
      .accounts({
        calculator: calculator.publicKey,
      })
      .rpc();
    const account = await program.account.calculator.fetch(
      calculator.publicKey
    );
    assert.ok(account.result.eq(new anchor.BN(4)));
  });

  it("Multiplications two numbers", async () => {
    await program.methods
      .mul(new anchor.BN(2), new anchor.BN(3))
      .accounts({
        calculator: calculator.publicKey,
      })
      .rpc();
    const account = await program.account.calculator.fetch(
      calculator.publicKey
    );
    assert.ok(account.result.eq(new anchor.BN(6)));
  });

  it("Divisions two numbers", async () => {
    await program.methods
      .div(new anchor.BN(5), new anchor.BN(2))
      .accounts({
        calculator: calculator.publicKey,
      })
      .rpc();
    const account = await program.account.calculator.fetch(
      calculator.publicKey
    );
    assert.ok(account.result.eq(new anchor.BN(2)));
    assert.ok(account.remainder.eq(new anchor.BN(1)));
  });
});
