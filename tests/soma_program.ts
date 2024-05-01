import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { SomaProgram } from "../target/types/soma_program";
import NodeWallet from "@coral-xyz/anchor/dist/cjs/nodewallet";
import { PublicKey, Keypair, LAMPORTS_PER_SOL } from "@solana/web3.js";
import { BN } from "bn.js";
import {
  ASSOCIATED_TOKEN_PROGRAM_ID,
  getOrCreateAssociatedTokenAccount,
} from "@solana/spl-token";
import { assert } from "chai";
import { sleep } from "./utils/utils";

describe("soma_protocol", () => {
  // Configure the client to use the local cluster.
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const program = anchor.workspace.SomaProgram as Program<SomaProgram>;

  const payer = (provider.wallet as NodeWallet).payer;

  const testUser = Keypair.generate();

  provider.connection.requestAirdrop(
    testUser.publicKey,
    10000 * LAMPORTS_PER_SOL
  );

  const TOKEN_22_PROGRAM = new PublicKey(
    "TokenzQdBNbLqP5VEhdkAS6EPFLC1PHnBqCXEpPxuEb"
  );

  const logUserStakeInfo = async (userStakeInfoPDA: PublicKey) => {
    await program.account.userStakeInfo
      .fetch(userStakeInfoPDA, "confirmed")
      .then((data) => {
        console.log(
          `userStakeInfo: { is_initialized: ${
            data.isInitialized
          }, tokenBalance: ${data.tokenBalance.toString()}, reward_balance: ${data.rewardBalance.toString()}, 
        stakeStartSlot: ${data.stakeStartSlot.toString()}, lastCalibrateSlot: ${data.lastCalibrateSlot.toString()}, 
        lockBalance: ${data.lockBalance.toString()}, lockRewardBalance: ${data.lockRewardBalance.toString()}, lockUntilSlot: ${data.lockUntilSlot.toString()} }`
        );
      });
  };

  const checkTokenBalance = async (
    name: string,
    tokenAccount: PublicKey,
    expected?: string
  ): Promise<anchor.web3.TokenAmount> => {
    const tokenAccountBalance =
      await provider.connection.getTokenAccountBalance(
        tokenAccount,
        "confirmed"
      );

    console.log(
      `[${name}] token balance: ${tokenAccountBalance.value.amount}, expected: ${expected}`
    );

    if (expected !== undefined) {
      assert.equal(tokenAccountBalance.value.amount, expected);
    }

    return tokenAccountBalance.value;
  };

  it("initialized program!", async () => {
    // return;
    // Add your test here.
    const [ownerConfigPDA] = PublicKey.findProgramAddressSync(
      [anchor.utils.bytes.utf8.encode("owner_config")],
      program.programId
    );

    const [relayerConfigPDA] = PublicKey.findProgramAddressSync(
      [anchor.utils.bytes.utf8.encode("relayer_config")],
      program.programId
    );

    const [minterPDA] = PublicKey.findProgramAddressSync(
      [anchor.utils.bytes.utf8.encode("minter")],
      program.programId
    );

    const [vaultPDA] = PublicKey.findProgramAddressSync(
      [anchor.utils.bytes.utf8.encode("vault")],
      program.programId
    );

    const [stakePoolPDA] = PublicKey.findProgramAddressSync(
      [anchor.utils.bytes.utf8.encode("stake_pool")],
      program.programId
    );

    console.log("payer", payer.publicKey.toString());
    console.log("ownerConfigPDA", ownerConfigPDA.toString());
    console.log("relayerConfigPDA", relayerConfigPDA.toString());
    console.log("minterPDA", minterPDA.toString());
    console.log("vaultPDA", vaultPDA.toString());
    console.log("stakePoolPDA", stakePoolPDA.toString());
    console.log("TOKEN_22_PROGRAM", TOKEN_22_PROGRAM.toString());

    const tx = await program.methods
      .initializeProgram({
        owner: payer.publicKey,
        relayer: payer.publicKey,
      })
      .accounts({
        payer: payer.publicKey,
        ownerConfig: ownerConfigPDA,
        relayerConfig: relayerConfigPDA,
        minter: minterPDA,
        vault: vaultPDA,
        stakePool: stakePoolPDA,
      })
      .rpc({ skipPreflight: true });
    console.log("Your transaction signature", tx);
  });

  it("initialized token!", async () => {
    // return;
    // Add your test here.
    const [ownerConfigPDA] = PublicKey.findProgramAddressSync(
      [anchor.utils.bytes.utf8.encode("owner_config")],
      program.programId
    );

    const [tokenConfigPDA] = PublicKey.findProgramAddressSync(
      [anchor.utils.bytes.utf8.encode("token_config")],
      program.programId
    );

    const [minterPDA] = PublicKey.findProgramAddressSync(
      [anchor.utils.bytes.utf8.encode("minter")],
      program.programId
    );

    const [mintPDA] = PublicKey.findProgramAddressSync(
      [anchor.utils.bytes.utf8.encode("mint_token:soma")],
      program.programId
    );

    const [vaultPDA] = PublicKey.findProgramAddressSync(
      [anchor.utils.bytes.utf8.encode("vault")],
      program.programId
    );

    const [vaultTokenAccountPDA] = PublicKey.findProgramAddressSync(
      [anchor.utils.bytes.utf8.encode("vault_token_account")],
      program.programId
    );

    const [stakePoolPDA] = PublicKey.findProgramAddressSync(
      [anchor.utils.bytes.utf8.encode("stake_pool")],
      program.programId
    );

    const [stakePoolTokenAccountPDA] = PublicKey.findProgramAddressSync(
      [anchor.utils.bytes.utf8.encode("stake_pool_token_account")],
      program.programId
    );

    console.log("payer", payer.publicKey.toString());
    console.log("ownerConfigPDA", ownerConfigPDA.toString());
    console.log("tokenConfigPDA", tokenConfigPDA.toString());
    console.log("minterPDA", minterPDA.toString());
    console.log("mintPDA", mintPDA.toString());
    console.log("vaultPDA", vaultPDA.toString());
    console.log("vaultTokenAccountPDA", vaultTokenAccountPDA.toString());
    console.log("stakePoolPDA", stakePoolPDA.toString());
    console.log(
      "stakePoolTokenAccountPDA",
      stakePoolTokenAccountPDA.toString()
    );
    console.log("TOKEN_22_PROGRAM", TOKEN_22_PROGRAM.toString());

    const tx = await program.methods
      .initializeToken({
        tokenDecimals: 4,
      })
      .accounts({
        payer: payer.publicKey,
        ownerConfig: ownerConfigPDA,
        tokenConfig: tokenConfigPDA,
        minter: minterPDA,
        mint: mintPDA,
        vault: vaultPDA,
        vaultTokenAccount: vaultTokenAccountPDA,
        stakePool: stakePoolPDA,
        stakePoolTokenAccount: stakePoolTokenAccountPDA,
        tokenProgram: TOKEN_22_PROGRAM,
      })
      .rpc({ skipPreflight: true });
    console.log("Your transaction signature", tx);
  });

  it("change stake rate", async () => {
    const [ownerConfigPDA] = PublicKey.findProgramAddressSync(
      [anchor.utils.bytes.utf8.encode("owner_config")],
      program.programId
    );

    const [stakePoolPDA] = PublicKey.findProgramAddressSync(
      [anchor.utils.bytes.utf8.encode("stake_pool")],
      program.programId
    );

    const tx = await program.methods
      .changeStakeRate(100_000_000) // 100 = 1% = actual rate 0.01
      .accounts({
        payer: payer.publicKey,
        ownerConfig: ownerConfigPDA,
        stakePool: stakePoolPDA,
      })
      .rpc({
        skipPreflight: true,
        commitment: "confirmed",
      });

    await program.account.stakePool.fetch(stakePoolPDA).then((data) => {
      console.log("after mint, vault info balance: ", data.balance.toString());
      assert.equal(data.stakeBaseRate, 100_000_000);
    });
  });

  it("mint to vault", async () => {
    const [relayerConfigPDA] = PublicKey.findProgramAddressSync(
      [anchor.utils.bytes.utf8.encode("relayer_config")],
      program.programId
    );

    const [tokenConfigPDA] = PublicKey.findProgramAddressSync(
      [anchor.utils.bytes.utf8.encode("token_config")],
      program.programId
    );

    const [minterPDA] = PublicKey.findProgramAddressSync(
      [anchor.utils.bytes.utf8.encode("minter")],
      program.programId
    );

    const [mintPDA] = PublicKey.findProgramAddressSync(
      [anchor.utils.bytes.utf8.encode("mint_token:soma")],
      program.programId
    );

    const [vaultPDA] = PublicKey.findProgramAddressSync(
      [anchor.utils.bytes.utf8.encode("vault")],
      program.programId
    );

    const [vaultTokenAccountPDA] = PublicKey.findProgramAddressSync(
      [anchor.utils.bytes.utf8.encode("vault_token_account")],
      program.programId
    );

    const tx = await program.methods
      .mintToVault(new BN(100000000))
      .accounts({
        payer: payer.publicKey,
        relayerConfig: relayerConfigPDA,
        tokenConfig: tokenConfigPDA,
        minter: minterPDA,
        mint: mintPDA,
        vault: vaultPDA,
        vaultTokenAccount: vaultTokenAccountPDA,
        tokenProgram: TOKEN_22_PROGRAM,
      })
      .rpc({
        skipPreflight: true,
        commitment: "confirmed",
      });

    await program.account.vault.fetch(vaultPDA).then((data) => {
      console.log("after mint, vault info balance: ", data.balance.toString());
      assert.isTrue(data.balance.eq(new BN(100000000)));
    });

    await checkTokenBalance("vault", vaultTokenAccountPDA, "100000000");
  });

  it("mint to user ata", async () => {
    const [relayerConfigPDA] = PublicKey.findProgramAddressSync(
      [anchor.utils.bytes.utf8.encode("relayer_config")],
      program.programId
    );

    const [tokenConfigPDA] = PublicKey.findProgramAddressSync(
      [anchor.utils.bytes.utf8.encode("token_config")],
      program.programId
    );

    const [minterPDA] = PublicKey.findProgramAddressSync(
      [anchor.utils.bytes.utf8.encode("minter")],
      program.programId
    );

    const [mintPDA] = PublicKey.findProgramAddressSync(
      [anchor.utils.bytes.utf8.encode("mint_token:soma")],
      program.programId
    );

    const userAta = await getOrCreateAssociatedTokenAccount(
      provider.connection,
      payer,
      mintPDA,
      testUser.publicKey,
      false,
      null,
      null,
      TOKEN_22_PROGRAM,
      ASSOCIATED_TOKEN_PROGRAM_ID
    );

    console.log("testUser", testUser.publicKey.toString());
    console.log("userAta", userAta.address.toString());

    const tx = await program.methods
      .mintToUserAta(new BN(100000000))
      .accounts({
        payer: payer.publicKey,
        relayerConfig: relayerConfigPDA,
        tokenConfig: tokenConfigPDA,
        minter: minterPDA,
        mint: mintPDA,
        user: testUser.publicKey,
        userTokenAccount: userAta.address,
        tokenProgram: TOKEN_22_PROGRAM,
        // associatedTokenProgram: ASSOCIATED_PROGRAM_ID,
      })
      .rpc({
        skipPreflight: true,
        commitment: "confirmed",
      });

    await checkTokenBalance("user", userAta.address, "100000000");
  });

  it("burn from vault", async () => {
    const [relayerConfigPDA] = PublicKey.findProgramAddressSync(
      [anchor.utils.bytes.utf8.encode("relayer_config")],
      program.programId
    );

    const [tokenConfigPDA] = PublicKey.findProgramAddressSync(
      [anchor.utils.bytes.utf8.encode("token_config")],
      program.programId
    );

    const [minterPDA] = PublicKey.findProgramAddressSync(
      [anchor.utils.bytes.utf8.encode("minter")],
      program.programId
    );

    const [mintPDA] = PublicKey.findProgramAddressSync(
      [anchor.utils.bytes.utf8.encode("mint_token:soma")],
      program.programId
    );

    const [vaultPDA] = PublicKey.findProgramAddressSync(
      [anchor.utils.bytes.utf8.encode("vault")],
      program.programId
    );

    const [vaultTokenAccountPDA] = PublicKey.findProgramAddressSync(
      [anchor.utils.bytes.utf8.encode("vault_token_account")],
      program.programId
    );

    const tx = await program.methods
      .burnFromVault(new BN(50000000))
      .accounts({
        payer: payer.publicKey,
        relayerConfig: relayerConfigPDA,
        tokenConfig: tokenConfigPDA,
        minter: minterPDA,
        mint: mintPDA,
        vault: vaultPDA,
        vaultTokenAccount: vaultTokenAccountPDA,
        tokenProgram: TOKEN_22_PROGRAM,
      })
      .rpc({
        skipPreflight: true,
        commitment: "confirmed",
      });

    await program.account.vault.fetch(vaultPDA, "confirmed").then((data) => {
      console.log(
        "after burn, vault info balance count: ",
        data.balance.toString()
      );
      assert.isTrue(data.balance.eq(new BN(50000000)));
    });

    await checkTokenBalance("vault", vaultTokenAccountPDA, "50000000");
  });

  it("user stake", async () => {
    const [tokenConfigPDA] = PublicKey.findProgramAddressSync(
      [anchor.utils.bytes.utf8.encode("token_config")],
      program.programId
    );

    const [minterPDA] = PublicKey.findProgramAddressSync(
      [anchor.utils.bytes.utf8.encode("minter")],
      program.programId
    );

    const [mintPDA] = PublicKey.findProgramAddressSync(
      [anchor.utils.bytes.utf8.encode("mint_token:soma")],
      program.programId
    );

    const [stakePoolPDA] = PublicKey.findProgramAddressSync(
      [anchor.utils.bytes.utf8.encode("stake_pool")],
      program.programId
    );

    const [stakePoolTokenAccountPDA] = PublicKey.findProgramAddressSync(
      [anchor.utils.bytes.utf8.encode("stake_pool_token_account")],
      program.programId
    );

    const [userStakeInfoPDA] = PublicKey.findProgramAddressSync(
      [
        anchor.utils.bytes.utf8.encode("user_stake_info:"),
        testUser.publicKey.toBuffer(),
      ],
      program.programId
    );

    const userAta = await getOrCreateAssociatedTokenAccount(
      provider.connection,
      payer,
      mintPDA,
      testUser.publicKey,
      false,
      null,
      null,
      TOKEN_22_PROGRAM,
      ASSOCIATED_TOKEN_PROGRAM_ID
    );

    console.log("testUser", testUser.publicKey.toString());
    console.log("userAta", userAta.address.toString());
    console.log("userStakeInfo", userStakeInfoPDA.toString());

    const tx = await program.methods
      .stake(new BN(70000000))
      .accounts({
        payer: testUser.publicKey,
        tokenConfig: tokenConfigPDA,
        minter: minterPDA,
        mint: mintPDA,
        stakePool: stakePoolPDA,
        stakePoolTokenAccount: stakePoolTokenAccountPDA,
        userStakeInfo: userStakeInfoPDA,
        userTokenAccount: userAta.address,
        tokenProgram: TOKEN_22_PROGRAM,
      })
      .signers([testUser])
      .rpc({
        skipPreflight: true,
        commitment: "confirmed",
      });

    {
      console.log("after 1st stake:");
      await logUserStakeInfo(userStakeInfoPDA);
      const userStakeInfo = await program.account.userStakeInfo.fetch(
        userStakeInfoPDA,
        "confirmed"
      );
      assert.isTrue(userStakeInfo.tokenBalance.eq(new BN("70000000")));

      await checkTokenBalance("user", userAta.address, "30000000");

      await program.account.stakePool
        .fetch(stakePoolPDA, "confirmed")
        .then((data) => {
          console.log("stakePool balance count: ", data.balance.toString());
          assert.isTrue(data.balance.eq(new BN("70000000")));
        });

      await checkTokenBalance(
        "stakePool",
        stakePoolTokenAccountPDA,
        "70000000"
      );
    }

    await sleep(5000); //wait for some slot

    const tx2 = await program.methods
      .stake(new BN(10000000))
      .accounts({
        payer: testUser.publicKey,
        tokenConfig: tokenConfigPDA,
        minter: minterPDA,
        mint: mintPDA,
        stakePool: stakePoolPDA,
        stakePoolTokenAccount: stakePoolTokenAccountPDA,
        userStakeInfo: userStakeInfoPDA,
        userTokenAccount: userAta.address,
        tokenProgram: TOKEN_22_PROGRAM,
      })
      .signers([testUser])
      .rpc({
        skipPreflight: true,
        commitment: "confirmed",
      });

    {
      console.log("after 2nd stake:");

      await logUserStakeInfo(userStakeInfoPDA);
      const userStakeInfo = await program.account.userStakeInfo.fetch(
        userStakeInfoPDA,
        "confirmed"
      );
      assert.isTrue(userStakeInfo.tokenBalance.eq(new BN("80000000")));

      await checkTokenBalance("user", userAta.address, "20000000");

      await program.account.stakePool
        .fetch(stakePoolPDA, "confirmed")
        .then((data) => {
          console.log("stakePool balance count: ", data.balance.toString());
          assert.isTrue(data.balance.eq(new BN("80000000")));
        });

      await checkTokenBalance(
        "stakePool",
        stakePoolTokenAccountPDA,
        "80000000"
      );
    }
  });

  it("user unstake", async () => {
    const [stakePoolPDA] = PublicKey.findProgramAddressSync(
      [anchor.utils.bytes.utf8.encode("stake_pool")],
      program.programId
    );

    const [userStakeInfoPDA] = PublicKey.findProgramAddressSync(
      [
        anchor.utils.bytes.utf8.encode("user_stake_info:"),
        testUser.publicKey.toBuffer(),
      ],
      program.programId
    );

    const tx = await program.methods
      .unstake(new BN("30000000"))
      .accounts({
        payer: testUser.publicKey,
        stakePool: stakePoolPDA,
        userStakeInfo: userStakeInfoPDA,
      })
      .signers([testUser])
      .rpc({
        skipPreflight: true,
        commitment: "confirmed",
      });

    console.log("after unstake:");

    await logUserStakeInfo(userStakeInfoPDA);
    const userStakeInfo = await program.account.userStakeInfo.fetch(
      userStakeInfoPDA,
      "confirmed"
    );
    // assert.isTrue(userStakeInfo.tokenBalance.eq(new BN(0)));
    // assert.isTrue(userStakeInfo.rewardBalance.eq(new BN(0)));
    // assert.isTrue(userStakeInfo.lockBalance.eq(new BN(10000000)));
    // assert.isTrue(userStakeInfo.lockRewardBalance.eq(new BN(0)));
  });

  it("user withdraw", async () => {
    const [tokenConfigPDA] = PublicKey.findProgramAddressSync(
      [anchor.utils.bytes.utf8.encode("token_config")],
      program.programId
    );

    const [minterPDA] = PublicKey.findProgramAddressSync(
      [anchor.utils.bytes.utf8.encode("minter")],
      program.programId
    );

    const [mintPDA] = PublicKey.findProgramAddressSync(
      [anchor.utils.bytes.utf8.encode("mint_token:soma")],
      program.programId
    );

    const [vaultPDA] = PublicKey.findProgramAddressSync(
      [anchor.utils.bytes.utf8.encode("vault")],
      program.programId
    );

    const [vaultTokenAccountPDA] = PublicKey.findProgramAddressSync(
      [anchor.utils.bytes.utf8.encode("vault_token_account")],
      program.programId
    );

    const [stakePoolPDA] = PublicKey.findProgramAddressSync(
      [anchor.utils.bytes.utf8.encode("stake_pool")],
      program.programId
    );

    const [stakePoolTokenAccountPDA] = PublicKey.findProgramAddressSync(
      [anchor.utils.bytes.utf8.encode("stake_pool_token_account")],
      program.programId
    );

    const [userStakeInfoPDA] = PublicKey.findProgramAddressSync(
      [
        anchor.utils.bytes.utf8.encode("user_stake_info:"),
        testUser.publicKey.toBuffer(),
      ],
      program.programId
    );

    const userAta = await getOrCreateAssociatedTokenAccount(
      provider.connection,
      payer,
      mintPDA,
      testUser.publicKey,
      false,
      null,
      null,
      TOKEN_22_PROGRAM,
      ASSOCIATED_TOKEN_PROGRAM_ID
    );

    console.log("testUser", testUser.publicKey.toString());
    console.log("userAta", userAta.address.toString());
    console.log("userStakeInfo", userStakeInfoPDA.toString());

    const tx = await program.methods
      .withdraw()
      .accounts({
        payer: testUser.publicKey,
        tokenConfig: tokenConfigPDA,
        minter: minterPDA,
        mint: mintPDA,
        vault: vaultPDA,
        vaultTokenAccount: vaultTokenAccountPDA,
        stakePool: stakePoolPDA,
        stakePoolTokenAccount: stakePoolTokenAccountPDA,
        userStakeInfo: userStakeInfoPDA,
        userTokenAccount: userAta.address,
        tokenProgram: TOKEN_22_PROGRAM,
      })
      .signers([testUser])
      .rpc({
        skipPreflight: true,
        commitment: "confirmed",
      });

    console.log("after withdraw");

    await logUserStakeInfo(userStakeInfoPDA);
    const userStakeInfo = await program.account.userStakeInfo.fetch(
      userStakeInfoPDA,
      "confirmed"
    );
    // assert.isFalse(userStakeInfo.isInitialized);
    // assert.isTrue(userStakeInfo.tokenBalance.eq(new BN(0)));
    // assert.isTrue(userStakeInfo.rewardBalance.eq(new BN(0)));
    // assert.isTrue(userStakeInfo.lockBalance.eq(new BN(0)));
    // assert.isTrue(userStakeInfo.lockRewardBalance.eq(new BN(0)));

    await checkTokenBalance("user", userAta.address, "50000000");

    await program.account.stakePool
      .fetch(stakePoolPDA, "confirmed")
      .then((data) => {
        console.log("stakePool balance count: ", data.balance.toString());
        // assert.isTrue(data.balance.eq(new BN("50000000")));
      });

    await checkTokenBalance("stakePool", stakePoolTokenAccountPDA);
    // await checkTokenBalance("stakePool", stakePoolTokenAccountPDA, "5000000");

    await program.account.vault.fetch(vaultPDA, "confirmed").then((data) => {
      console.log("vault balance count: ", data.balance.toString());
      // assert.isTrue(data.balance.eq(new BN("5000000")));
    });

    await checkTokenBalance("vault", vaultTokenAccountPDA);
    // await checkTokenBalance("vault", vaultTokenAccountPDA, "5000000");
  });
});
