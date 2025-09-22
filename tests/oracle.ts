import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Oracle } from "../target/types/oracle";
import { Minter } from "../target/types/minter";
import { Keypair, PublicKey, SystemProgram } from "@solana/web3.js";
import { TOKEN_PROGRAM_ID } from "@coral-xyz/anchor/dist/cjs/utils/token";
import fs from "fs";
import { MPL_TOKEN_METADATA_PROGRAM_ID } from "@metaplex-foundation/mpl-token-metadata";
import {
  getMint,
  getAccount,
  getOrCreateAssociatedTokenAccount,
} from "@solana/spl-token";
import { BN } from "bn.js";
describe("oracle", () => {
  // Set up provider & program
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const program = anchor.workspace.Oracle as Program<Oracle>;
  const minterProgram = anchor.workspace.Minter as Program<Minter>;

  const wallet = provider.wallet as anchor.Wallet;
  const secret = JSON.parse(fs.readFileSync("wallet.json", "utf-8"));

  // Convert into a Keypair
  const keypair = Keypair.fromSecretKey(new Uint8Array(secret));
  const wallet2 = new anchor.Wallet(keypair);

  let oracleDataAccount: PublicKey;
  const minterProgramId = minterProgram.programId;// new PublicKey("AF1EgdUAE7NEopGSEatoyZ8qSg7Tt5Ct1EmZXnPwmyjA");
  const [mint, mintBump] = anchor.web3.PublicKey.findProgramAddressSync(
    [Buffer.from("mint")],
    minterProgramId,
  );
  const [operation, operationBump] = anchor.web3.PublicKey.findProgramAddressSync(
    [Buffer.from("operation")],
    minterProgramId,
  );
  const [tokenAccount, tokenAccountBump] = anchor.web3.PublicKey.findProgramAddressSync(
    [Buffer.from("token")],
    minterProgramId,
  );

  before(async () => {
    // Derive PDA for OracleData (fixed seed so it's persistent)
    [oracleDataAccount] = await PublicKey.findProgramAddress(
      [Buffer.from("oracle_data")],
      program.programId
    );

    // Initialize OracleData account (idempotent: skip if already exists)
    try {
      await program.methods.initializeOracle()
        .accounts({
          user: wallet.publicKey,

        })
    //   .rpc();

      //  console.log("OracleData initialized:", oracleDataAccount.toBase58());
    } catch (err: any) {
      if (err.error?.errorCode?.code === "AccountAlreadyInitialized") {
        console.log("OracleData already initialized, skipping.");
      } else {
        throw err;
      }
    }
  });

  it("Is initialize Minter !", async () => {
    console.log("token", mint.toBase58());
    const tx = await minterProgram.methods
      .initialize(program.programId)
      .accounts({
        signer: wallet.publicKey,
      })
      //.rpc({ commitment: "confirmed" });
   console.log("Your transaction signature", tx);

  //   // const mintccount = await getMint(
  //   //   program.provider.connection,
  //   //   mint,
  //   //   "confirmed",
  //   //   TOKEN_PROGRAM_ID,
  //   // );

  //   console.log("Mint ", mint);
  //   // console.log("mint address tka", token.toBase58());
  //   console.log("operation address tka", operation.toBase58());
  //   console.log("operations", await minterProgram.account.operation.fetch(operation));
  });


  it("Is update Minter status !", async () => {
    const tx = await minterProgram.methods
      .updateStatus(1)
      .accounts({
        admin: wallet.publicKey,
      })
      .rpc({ commitment: "confirmed" });
   console.log("Your transaction signature", tx);

  //   // const mintccount = await getMint(
  //   //   program.provider.connection,
  //   //   mint,
  //   //   "confirmed",
  //   //   TOKEN_PROGRAM_ID,
  //   // );

  //   console.log("Mint ", mint);
  //   // console.log("mint address tka", token.toBase58());
  //   console.log("operation address tka", operation.toBase58());
  //   console.log("operations", await minterProgram.account.operation.fetch(operation));
  });
  // it("Is update oracle !", async () => {
  //   console.log("oracle data account", oracleDataAccount.toBase58());
  //   const tx = await minterProgram.methods
  //     .updateOracle(oracleDataAccount)
  //     .accounts({
  //       admin: wallet2.publicKey,
  //     }).signers([keypair])
  //     .rpc();
  //   console.log("Your transaction signature", tx);


  //   console.log("Mint ", mint);
  //   // console.log("mint address tka", token.toBase58());
  //   console.log("operation address tka", operation.toBase58());
  //   console.log("operations", await minterProgram.account.operation.fetch(operation));
  // });


  it("Adds a pulse!", async () => {
    // Fetch current oracle data
    const data = await program.account.oracleData.fetch(oracleDataAccount);
    console.log("data", data.latestPulse.toNumber());

    const nextPulseId = new anchor.BN(data.latestPulse.toNumber() + 1);

    // // Derive PDA for oraclePulse using nextPulseId
    const [oraclePulse] = await PublicKey.findProgramAddress(
      [Buffer.from("oracle"), Buffer.from((nextPulseId.toNumber()).toString())], // little-endian u64
      program.programId
    );

    // Add pulse
    const tx = await program.methods
      .addPulse(new anchor.BN(1000000000)) // available_balance 
      .accounts({
        oraclePulse,
        mint,
        admin: wallet.publicKey,
      })
     .rpc();

    console.log("Pulse tx signature:", tx);

    // Verify the new pulse account
    const pulse = await program.account.oraclePulse.fetch(oraclePulse);
    console.log("New pulse created:", pulse);

    // Verify OracleData updated
    const updatedData = await program.account.oracleData.fetch(oracleDataAccount);
    console.log("Updated OracleData:", updatedData);

    // Assertions
    // if (!pulse) throw new Error("Pulse not created!");
    // if (updatedData.latestPulse.toNumber() !== nextPulseId.toNumber()) {
    //   throw new Error("OracleData.latestPulse was not updated correctly");
    // }
  });
  //   it("Transfer Tokens", async () => {
  //     const recipientTokenAccount = await getOrCreateAssociatedTokenAccount(program.provider.connection,program.provider.wallet.payer, mint, program.provider.wallet.publicKey);

  //   const tx = await minterProgram.methods.transferTokens(new anchor.BN(980000000)) // 1 token with 6 decimals
  //     .accounts({
  //       recipientTokenAccount: recipientTokenAccount.address,
  //     })
  //     .rpc({ commitment: "confirmed" });

  //   console.log("Your transaction signature", tx);
  // });

  //  it("creates metadata", async () => {
  //   // Generate a new mint (not initialized here – could add full mint setup if needed)
  //   const metadataProgram=new PublicKey("metaqbxxUerdq28cj1RbAWkYQm3ybzjb6a8bt518x1s");
  //   // Derive PDA for metadata
  //   const [metadataPda] = PublicKey.findProgramAddressSync(
  //     [
  //       Buffer.from("metadata"),
  //       metadataProgram.toBuffer(),
  //       mint.toBuffer(),
  //     ],
  //     metadataProgram
  
  //   );

  //   // Call your create_metadata instruction
  //   await minterProgram.methods.createMetadata("Momo USD", "Musd", "https://olive-obvious-turkey-318.mypinata.cloud/ipfs/bafkreif5yieq4mmmhakf3qgka3k5kletgdv2xyfpd2ruwg5zl344e6acie")
  //     .accounts({
  //               payer: provider.wallet.publicKey,
  //               metadata: metadataPda,

  //     }).signers([provider.wallet.payer])
  //     .rpc();

  //   console.log("✅ Metadata created:", metadataPda.toBase58());
  // });
});
