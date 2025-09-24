// import * as anchor from "@coral-xyz/anchor";
// import { Program } from "@coral-xyz/anchor";
// import { Oracle } from "../target/types/oracle";
// import { Minter } from "../target/types/minter";
// import { Keypair, PublicKey, SystemProgram } from "@solana/web3.js";
// import { TOKEN_PROGRAM_ID } from "@coral-xyz/anchor/dist/cjs/utils/token";
// import fs from "fs";
// import { MPL_TOKEN_METADATA_PROGRAM_ID } from "@metaplex-foundation/mpl-token-metadata";
// import {
//   getMint,
//   getAccount,
//   getOrCreateAssociatedTokenAccount,
// } from "@solana/spl-token";
// import { BN } from "bn.js";
// describe("oracle", () => {
//   // Set up provider & program
//   const provider = anchor.AnchorProvider.env();
//   anchor.setProvider(provider);

//   const program = anchor.workspace.Oracle as Program<Oracle>;
//   const minterProgram = anchor.workspace.Minter as Program<Minter>;

//   const wallet = provider.wallet as anchor.Wallet;
//   const secret = JSON.parse(fs.readFileSync("wallet.json", "utf-8"));

//   // Convert into a Keypair
//   const keypair = Keypair.fromSecretKey(new Uint8Array(secret));
//   const wallet2 = new anchor.Wallet(keypair);

//   let oracleDataAccount: PublicKey;
//   const minterProgramId = minterProgram.programId;// new PublicKey("AF1EgdUAE7NEopGSEatoyZ8qSg7Tt5Ct1EmZXnPwmyjA");
//   const [mint, mintBump] = anchor.web3.PublicKey.findProgramAddressSync(
//     [Buffer.from("mint")],
//     minterProgramId,
//   );
//   const [operation, operationBump] = anchor.web3.PublicKey.findProgramAddressSync(
//     [Buffer.from("operation")],
//     minterProgramId,
//   );
//   const [tokenAccount, tokenAccountBump] = anchor.web3.PublicKey.findProgramAddressSync(
//     [Buffer.from("token")],
//     minterProgramId,
//   );

//   before(async () => {
//     // Derive PDA for OracleData (fixed seed so it's persistent)
//     [oracleDataAccount] = await PublicKey.findProgramAddress(
//       [Buffer.from("oracle_data")],
//       program.programId
//     );

//     // Initialize OracleData account (idempotent: skip if already exists)
//     try {
//       await program.methods.initializeOracle()
//         .accounts({
//           user: wallet.publicKey,

//         })
//     //   .rpc();

//       //  console.log("OracleData initialized:", oracleDataAccount.toBase58());
//     } catch (err: any) {
//       if (err.error?.errorCode?.code === "AccountAlreadyInitialized") {
//         console.log("OracleData already initialized, skipping.");
//       } else {
//         throw err;
//       }
//     }
//   });

//   it("Is initialize Minter !", async () => {
//     console.log("token", mint.toBase58());
//     const tx = await minterProgram.methods
//       .initialize(program.programId)
//       .accounts({
//         signer: wallet.publicKey,
//       })
//       //.rpc({ commitment: "confirmed" });
//    console.log("Your transaction signature", tx);

//   //   // const mintccount = await getMint(
//   //   //   program.provider.connection,
//   //   //   mint,
//   //   //   "confirmed",
//   //   //   TOKEN_PROGRAM_ID,
//   //   // );

//   //   console.log("Mint ", mint);
//   //   // console.log("mint address tka", token.toBase58());
//   //   console.log("operation address tka", operation.toBase58());
//   //   console.log("operations", await minterProgram.account.operation.fetch(operation));
//   });


//   it("Is update Minter status !", async () => {
//     const tx = await minterProgram.methods
//       .updateStatus(1)
//       .accounts({
//         admin: wallet.publicKey,
//       })
//       .rpc({ commitment: "confirmed" });
//    console.log("Your transaction signature", tx);

//   //   // const mintccount = await getMint(
//   //   //   program.provider.connection,
//   //   //   mint,
//   //   //   "confirmed",
//   //   //   TOKEN_PROGRAM_ID,
//   //   // );

//   //   console.log("Mint ", mint);
//   //   // console.log("mint address tka", token.toBase58());
//   //   console.log("operation address tka", operation.toBase58());
//   //   console.log("operations", await minterProgram.account.operation.fetch(operation));
//   });
//   it("Is update oracle !", async () => {
//     console.log("oracle data account", oracleDataAccount.toBase58());
//     const tx = await minterProgram.methods
//       .updateOracle(program.programId)
//       .accounts({
//         admin: wallet.publicKey,
//       }).signers([])
//       .rpc();
//     console.log("Your transaction signature", tx);


//     console.log("Mint ", mint);
//     // console.log("mint address tka", token.toBase58());
//     console.log("operation address tka", operation.toBase58());
//     console.log("operations", await minterProgram.account.operation.fetch(operation));
//   });


//   it("Adds a pulse!", async () => {
//     // Fetch current oracle data
//     const data = await program.account.oracleData.fetch(oracleDataAccount);
//     console.log("data", data.latestPulse.toNumber());

//     const nextPulseId = new anchor.BN(data.latestPulse.toNumber() + 1);

//     // // Derive PDA for oraclePulse using nextPulseId
//     const [oraclePulse] = await PublicKey.findProgramAddress(
//       [Buffer.from("oracle"), Buffer.from((nextPulseId.toNumber()).toString())], // little-endian u64
//       program.programId
//     );

//     // Add pulse
//     const tx = await program.methods
//       .addPulse(new anchor.BN(1000000000)) // available_balance 
//       .accounts({
//         oraclePulse,
//         mint,
//         admin: wallet.publicKey,
//       })
//      .rpc();

//     console.log("Pulse tx signature:", tx);

//     // Verify the new pulse account
//     const pulse = await program.account.oraclePulse.fetch(oraclePulse);
//     console.log("New pulse created:", pulse);

//     // Verify OracleData updated
//     const updatedData = await program.account.oracleData.fetch(oracleDataAccount);
//     console.log("Updated OracleData:", updatedData);

//     // Assertions
//     // if (!pulse) throw new Error("Pulse not created!");
//     // if (updatedData.latestPulse.toNumber() !== nextPulseId.toNumber()) {
//     //   throw new Error("OracleData.latestPulse was not updated correctly");
//     // }
//   });
//   //   it("Transfer Tokens", async () => {
//   //     const recipientTokenAccount = await getOrCreateAssociatedTokenAccount(program.provider.connection,program.provider.wallet.payer, mint, program.provider.wallet.publicKey);

//   //   const tx = await minterProgram.methods.transferTokens(new anchor.BN(980000000)) // 1 token with 6 decimals
//   //     .accounts({
//   //       recipientTokenAccount: recipientTokenAccount.address,
//   //     })
//   //     .rpc({ commitment: "confirmed" });

//   //   console.log("Your transaction signature", tx);
//   // });

//   //  it("creates metadata", async () => {
//   //   // Generate a new mint (not initialized here – could add full mint setup if needed)
//   //   const metadataProgram=new PublicKey("metaqbxxUerdq28cj1RbAWkYQm3ybzjb6a8bt518x1s");
//   //   // Derive PDA for metadata
//   //   const [metadataPda] = PublicKey.findProgramAddressSync(
//   //     [
//   //       Buffer.from("metadata"),
//   //       metadataProgram.toBuffer(),
//   //       mint.toBuffer(),
//   //     ],
//   //     metadataProgram
  
//   //   );

//   //   // Call your create_metadata instruction
//   //   await minterProgram.methods.createMetadata("Momo USD", "Musd", "https://olive-obvious-turkey-318.mypinata.cloud/ipfs/bafkreif5yieq4mmmhakf3qgka3k5kletgdv2xyfpd2ruwg5zl344e6acie")
//   //     .accounts({
//   //               payer: provider.wallet.publicKey,
//   //               metadata: metadataPda,

//   //     }).signers([provider.wallet.payer])
//   //     .rpc();

//   //   console.log("✅ Metadata created:", metadataPda.toBase58());
//   // });
// });




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
import { expect } from "chai";

describe("Oracle and Minter Integration Tests", () => {
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
  const minterProgramId = minterProgram.programId;
  
  // PDAs for minter program
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

  // Test data
  let initialSupply = 0;
  let pulseCounter = 0;

  before(async () => {
    console.log("Setting up test environment...");
    
    // Derive PDA for OracleData
    [oracleDataAccount] = await PublicKey.findProgramAddress(
      [Buffer.from("oracle_data")],
      program.programId
    );

    console.log("Oracle Data Account:", oracleDataAccount.toBase58());
    console.log("Mint:", mint.toBase58());
    console.log("Operation:", operation.toBase58());
    console.log("Token Account:", tokenAccount.toBase58());
  });

  describe("Initialization Tests", () => {
    it("Should initialize Oracle Data account", async () => {
      try {
        const tx = await program.methods.initializeOracle()
          .accounts({
            user: wallet.publicKey,
          })
          .rpc();

        console.log("Oracle initialization tx:", tx);

        // Verify oracle data account
        const data = await program.account.oracleData.fetch(oracleDataAccount);
        expect(data.latestPulse.toNumber()).to.equal(0);
        expect(data.admin.toBase58()).to.equal(wallet.publicKey.toBase58());
        
      } catch (err: any) {
        if (err.error?.errorCode?.code === "AccountAlreadyInitialized") {
          console.log("Oracle already initialized, verifying existing data...");
          const data = await program.account.oracleData.fetch(oracleDataAccount);
          expect(data.admin.toBase58()).to.equal(wallet.publicKey.toBase58());
        } else {
          throw err;
        }
      }
    });

    it("Should initialize Minter program", async () => {
      const tx = await minterProgram.methods
        .initialize(program.programId)
        .accounts({
          signer: wallet.publicKey,
        })
        .rpc({ commitment: "confirmed" });

      console.log("Minter initialization tx:", tx);

      // Verify mint account
      const mintAccount = await getMint(
        provider.connection,
        mint,
        "confirmed",
        TOKEN_PROGRAM_ID,
      );
      expect(mintAccount.decimals).to.equal(6);
      expect(mintAccount.mintAuthority?.toBase58()).to.equal(mint.toBase58());

      // Verify operation account
      const operationData = await minterProgram.account.operation.fetch(operation);
      expect(operationData.admin.toBase58()).to.equal(wallet.publicKey.toBase58());
      expect(operationData.oracle.toBase58()).to.equal(program.programId.toBase58());
      expect(operationData.status).to.equal(0); // paused

      // Verify token account
      const tokenAccountData = await getAccount(
        provider.connection,
        tokenAccount,
        "confirmed",
        TOKEN_PROGRAM_ID,
      );
      expect(tokenAccountData.mint.toBase58()).to.equal(mint.toBase58());
      expect(tokenAccountData.amount.toString()).to.equal("0");

      initialSupply = Number(mintAccount.supply);
    });
  });

  describe("Admin Operations Tests", () => {
    it("Should update minter status to active", async () => {
      const tx = await minterProgram.methods
        .updateStatus(1) // active
        .accounts({
          admin: wallet.publicKey,
        })
        .rpc({ commitment: "confirmed" });

      console.log("Status update tx:", tx);

      const operationData = await minterProgram.account.operation.fetch(operation);
      expect(operationData.status).to.equal(1);
    });

    it("Should fail to update status with invalid value", async () => {
      try {
        await minterProgram.methods
          .updateStatus(2) // invalid status
          .accounts({
            admin: wallet.publicKey,
          })
          .rpc({ commitment: "confirmed" });
        
        throw new Error("Should have failed with invalid status");
      } catch (err: any) {
        expect(err.error.errorCode.code).to.equal("InvalidStatus");
      }
    });

    it("Should update oracle address", async () => {
      const tx = await minterProgram.methods
        .updateOracle(program.programId)
        .accounts({
          admin: wallet.publicKey,
        })
        .rpc();

      console.log("Oracle update tx:", tx);

      const operationData = await minterProgram.account.operation.fetch(operation);
      expect(operationData.oracle.toBase58()).to.equal(program.programId.toBase58());
    });
   it("Should update cool down time", async () => {
      const tx = await minterProgram.methods
        .updateCoolDownPeriodInSeconds(new BN(3600))
        .accounts({
          admin: wallet.publicKey,
        })
        .rpc();

      console.log("Oracle update tx:", tx);

      const operationData = await minterProgram.account.operation.fetch(operation);
      expect(operationData.oracle.toBase58()).to.equal(program.programId.toBase58());
    });

    it("Should fail admin operations with unauthorized user", async () => {
      try {
        await minterProgram.methods
          .updateStatus(0)
          .accounts({
            admin: wallet2.publicKey,
          })
          .signers([keypair])
          .rpc();
        
        throw new Error("Should have failed with unauthorized user");
      } catch (err: any) {
        expect(err.error.errorCode.code).to.equal("UnauthorizedAdminUser");
      }
    });

    it("Should transfer admin rights", async () => {
      // Transfer admin to wallet2
      const tx1 = await minterProgram.methods
        .updateAdmin(wallet2.publicKey)
        .accounts({
          admin: wallet.publicKey,
        })
        .rpc();

      console.log("Admin transfer tx:", tx1);

      let operationData = await minterProgram.account.operation.fetch(operation);
      expect(operationData.admin.toBase58()).to.equal(wallet2.publicKey.toBase58());

      // Transfer back to original wallet
      const tx2 = await minterProgram.methods
        .updateAdmin(wallet.publicKey)
        .accounts({
          admin: wallet2.publicKey,
        })
        .signers([keypair])
        .rpc();

      console.log("Admin transfer back tx:", tx2);

      operationData = await minterProgram.account.operation.fetch(operation);
      expect(operationData.admin.toBase58()).to.equal(wallet.publicKey.toBase58());
    });
  });

  describe("Oracle Pulse Tests", () => {
    it("Should add pulse when minter is paused", async () => {
      // First, pause the minter
      await minterProgram.methods
        .updateStatus(0) // paused
        .accounts({
          admin: wallet.publicKey,
        })
        .rpc({ commitment: "confirmed" });

      const data = await program.account.oracleData.fetch(oracleDataAccount);
      const nextPulseId = new anchor.BN(data.latestPulse.toNumber() + 1);
      pulseCounter = nextPulseId.toNumber();

      const [oraclePulse] = await PublicKey.findProgramAddress(
        [Buffer.from("oracle"), Buffer.from(nextPulseId.toString())],
        program.programId
      );

      const tx = await program.methods
        .addPulse(new anchor.BN(1000000000)) // 1000 tokens
        .accounts({
          oraclePulse,
          mint,
          admin: wallet.publicKey,
        })
        .rpc();

      console.log("Pulse tx (paused):", tx);

      const pulse = await program.account.oraclePulse.fetch(oraclePulse);
      expect(pulse.availableBankBalance.toNumber()).to.equal(1000000000);
      expect(pulse.tokenOperationLog).to.include("Operation is paused");

      const updatedData = await program.account.oracleData.fetch(oracleDataAccount);
      expect(updatedData.latestPulse.toNumber()).to.equal(nextPulseId.toNumber());
    });

    it("Should mint tokens when available balance > supply", async () => {
      // Activate minter
      await minterProgram.methods
        .updateStatus(1)
        .accounts({
          admin: wallet.publicKey,
        })
        .rpc({ commitment: "confirmed" });

      const data = await program.account.oracleData.fetch(oracleDataAccount);
      const nextPulseId = new anchor.BN(data.latestPulse.toNumber() + 1);
      pulseCounter = nextPulseId.toNumber();

      const [oraclePulse] = await PublicKey.findProgramAddress(
        [Buffer.from("oracle"), Buffer.from(nextPulseId.toString())],
        program.programId
      );

      const mintBefore = await getMint(provider.connection, mint, "confirmed");
      const supplyBefore = mintBefore.supply;

      const availableBalance = 200000000000; // 200000 tokens
      const tx = await program.methods
        .addPulse(new anchor.BN(availableBalance))
        .accounts({
          oraclePulse,
          mint,
          admin: wallet.publicKey,
        })
        .rpc();

      console.log("Pulse tx (mint):", tx);

      const pulse = await program.account.oraclePulse.fetch(oraclePulse);
      expect(pulse.tokenOperationType).to.equal(1); // mint operation
      expect(pulse.tokenOperationLog).to.include("Minted");

      const mintAfter = await getMint(provider.connection, mint, "confirmed");
      const expectedSupply = availableBalance;
      expect(mintAfter.supply.toString()).to.equal(expectedSupply.toString());
    });

    it("Should burn tokens when available balance < supply", async () => {
      const data = await program.account.oracleData.fetch(oracleDataAccount);
      const nextPulseId = new anchor.BN(data.latestPulse.toNumber() + 1);
      pulseCounter = nextPulseId.toNumber();

      const [oraclePulse] = await PublicKey.findProgramAddress(
        [Buffer.from("oracle"), Buffer.from(nextPulseId.toString())],
        program.programId
      );

      const mintBefore = await getMint(provider.connection, mint, "confirmed");
      const supplyBefore = mintBefore.supply;

      const availableBalance = 1500000000; // 1500 tokens (less than current supply)
      const tx = await program.methods
        .addPulse(new anchor.BN(availableBalance))
        .accounts({
          oraclePulse,
          mint,
          admin: wallet.publicKey,
        })
        .rpc();

      console.log("Pulse tx (burn):", tx);

      const pulse = await program.account.oraclePulse.fetch(oraclePulse);
      expect(pulse.tokenOperationType).to.equal(2); // burn operation
      expect(pulse.tokenOperationLog).to.include("Burned");

      const mintAfter = await getMint(provider.connection, mint, "confirmed");
      expect(Number(mintAfter.supply)).to.be.lessThan(Number(supplyBefore));
    });

    it("Should handle case when no operation is needed", async () => {
      const mintCurrent = await getMint(provider.connection, mint, "confirmed");
      const currentSupply = Number(mintCurrent.supply);

      const data = await program.account.oracleData.fetch(oracleDataAccount);
      const nextPulseId = new anchor.BN(data.latestPulse.toNumber() + 1);

      const [oraclePulse] = await PublicKey.findProgramAddress(
        [Buffer.from("oracle"), Buffer.from(nextPulseId.toString())],
        program.programId
      );

      const tx = await program.methods
        .addPulse(new anchor.BN(currentSupply)) // same as current supply
        .accounts({
          oraclePulse,
          mint,
          admin: wallet.publicKey,
        })
        .rpc();

      console.log("Pulse tx (no operation):", tx);

      const pulse = await program.account.oraclePulse.fetch(oraclePulse);
      expect(pulse.tokenOperationLog).to.include("No operation needed");
    });

    it("Should fail pulse with unauthorized user", async () => {
      const data = await program.account.oracleData.fetch(oracleDataAccount);
      const nextPulseId = new anchor.BN(data.latestPulse.toNumber() + 1);

      const [oraclePulse] = await PublicKey.findProgramAddress(
        [Buffer.from("oracle"), Buffer.from(nextPulseId.toString())],
        program.programId
      );

      try {
        await program.methods
          .addPulse(new anchor.BN(1000000000))
          .accounts({
            oraclePulse,
            mint,
            admin: wallet2.publicKey,
          })
          .signers([keypair])
          .rpc();
        
        throw new Error("Should have failed with unauthorized user");
      } catch (err: any) {
        expect(err.error.errorCode.code).to.equal("UnAuthorizedUser");
      }
    });
  });

  describe("Token Operations Tests", () => {
    let userTokenAccount: any;

    before(async () => {
      userTokenAccount = await getOrCreateAssociatedTokenAccount(
        provider.connection,
        wallet.payer,
        mint,
        wallet.publicKey
      );
    });

    it("Should transfer tokens to user", async () => {
      const transferAmount = 10; // 100 tokens

      const balanceBefore = await getAccount(
        provider.connection,
        userTokenAccount.address,
        "confirmed"
      );

      const tx = await minterProgram.methods
        .transferTokens(new anchor.BN(transferAmount))
        .accounts({
          recipientTokenAccount: userTokenAccount.address,
        })
        .rpc({ commitment: "confirmed" });

      console.log("Transfer tx:", tx);

      const balanceAfter = await getAccount(
        provider.connection,
        userTokenAccount.address,
        "confirmed"
      );

      expect(Number(balanceAfter.amount) - Number(balanceBefore.amount)).to.equal(transferAmount);
    });

    it("Should get token supply stats", async () => {
      const supply = await minterProgram.methods
        .getStatsSupply()
        .accounts({
          mint,
          tokenAccount,
          operation,
          oracleProgram: program.programId,
        })
        .rpc();

      console.log("Current supply:", supply.toString());

      const mintData = await getMint(provider.connection, mint, "confirmed");
      expect(supply.toString()).to.equal(mintData.supply.toString());
    });
  });

  describe("Metadata Tests", () => {
    it("Should create token metadata", async () => {
      const metadataProgram = new PublicKey("metaqbxxUerdq28cj1RbAWkYQm3ybzjb6a8bt518x1s");
      
      const [metadataPda] = PublicKey.findProgramAddressSync(
        [
          Buffer.from("metadata"),
          metadataProgram.toBuffer(),
          mint.toBuffer(),
        ],
        metadataProgram
      );

      try {
        const tx = await minterProgram.methods
          .createMetadata(
            "Test USD", 
            "TUSD", 
            "https://example.com/metadata.json"
          )
          .accounts({
            payer: wallet.publicKey,
            metadata: metadataPda,
          })
          .rpc();

        console.log("Metadata creation tx:", tx);
        console.log("Metadata PDA:", metadataPda.toBase58());

      } catch (err: any) {
        // Metadata might already exist, that's okay
        if (!err.message.includes("already in use")) {
          throw err;
        }
        console.log("Metadata already exists");
      }
    });
  });

  describe("Edge Cases and Error Handling", () => {
    it("Should handle burn when token account has insufficient balance", async () => {
      // First, let's get current token account balance
      const tokenAccountData = await getAccount(
        provider.connection,
        tokenAccount,
        "confirmed"
      );
      
      const data = await program.account.oracleData.fetch(oracleDataAccount);
      const nextPulseId = new anchor.BN(data.latestPulse.toNumber() + 1);

      const [oraclePulse] = await PublicKey.findProgramAddress(
        [Buffer.from("oracle"), Buffer.from(nextPulseId.toString())],
        program.programId
      );

      // Set available balance to 0 to force a large burn
      const tx = await program.methods
        .addPulse(new anchor.BN(0))
        .accounts({
          oraclePulse,
          mint,
          admin: wallet.publicKey,
        })
        .rpc();

      console.log("Pulse tx (insufficient burn):", tx);

      const pulse = await program.account.oraclePulse.fetch(oraclePulse);
      
      // Should handle the insufficient balance gracefully
      if (tokenAccountData.amount === BigInt(0)) {
        expect(pulse.tokenOperationLog).to.include("No tokens available");
      } else {
        expect(pulse.tokenOperationLog).to.include("only");
      }
    });

    it("Should handle multiple sequential pulses", async () => {
      const pulses = [500000000, 750000000, 1000000000]; // Different amounts
      
      for (let i = 0; i < pulses.length; i++) {
        const data = await program.account.oracleData.fetch(oracleDataAccount);
        const nextPulseId = new anchor.BN(data.latestPulse.toNumber() + 1);

        const [oraclePulse] = await PublicKey.findProgramAddress(
          [Buffer.from("oracle"), Buffer.from(nextPulseId.toString())],
          program.programId
        );

        const tx = await program.methods
          .addPulse(new anchor.BN(pulses[i]))
          .accounts({
            oraclePulse,
            mint,
            admin: wallet.publicKey,
          })
          .rpc();

        console.log(`Sequential pulse ${i + 1} tx:`, tx);

        const pulse = await program.account.oraclePulse.fetch(oraclePulse);
        expect(pulse.availableBankBalance.toNumber()).to.equal(pulses[i]);
        expect(pulse.pulse.toNumber()).to.equal(nextPulseId.toNumber());
      }
    });
  });

  describe("Final State Verification", () => {
    it("Should verify final oracle state", async () => {
      const oracleData = await program.account.oracleData.fetch(oracleDataAccount);
      
      console.log("Final Oracle State:");
      console.log("- Latest Pulse:", oracleData.latestPulse.toNumber());
      console.log("- Last Updated:", new Date(oracleData.lastUpdated.toNumber() * 1000));
      console.log("- Admin:", oracleData.admin.toBase58());

      expect(oracleData.latestPulse.toNumber()).to.be.greaterThan(0);
      expect(oracleData.admin.toBase58()).to.equal(wallet.publicKey.toBase58());
    });

    it("Should verify final minter state", async () => {
      const operationData = await minterProgram.account.operation.fetch(operation);
      const mintData = await getMint(provider.connection, mint, "confirmed");
      const tokenAccountData = await getAccount(provider.connection, tokenAccount, "confirmed");

      console.log("Final Minter State:");
      console.log("- Operation Status:", operationData.status);
      console.log("- Total Supply:", mintData.supply.toString());
      console.log("- Token Account Balance:", tokenAccountData.amount.toString());
      console.log("- Admin:", operationData.admin.toBase58());
      console.log("- Oracle:", operationData.oracle.toBase58());

      expect(operationData.status).to.be.oneOf([0, 1]);
      expect(operationData.admin.toBase58()).to.equal(wallet.publicKey.toBase58());
    });
  });
});