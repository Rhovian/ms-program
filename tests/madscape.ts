import * as anchor from "@coral-xyz/anchor";
import { Program, web3 } from "@coral-xyz/anchor";
import {
  ASSOCIATED_TOKEN_PROGRAM_ID,
  createAssociatedTokenAccount,
  createMint,
  getAssociatedTokenAddress,
  getOrCreateAssociatedTokenAccount,
  mintTo,
  TOKEN_PROGRAM_ID,
} from "@solana/spl-token";
import { TextEncoder } from "util";
import { expect } from "chai";

import { Madscape } from "../target/types/madscape";
import NodeWallet from "@coral-xyz/anchor/dist/cjs/nodewallet";

const encode = (str: string) => new TextEncoder().encode(str);
const b = (input: TemplateStringsArray) => encode(input.join(""));

const buildReleaseAuthorityPDA = (
  authority: web3.PublicKey,
  programId: web3.PublicKey
) =>
  web3.PublicKey.findProgramAddressSync(
    [b`release`, authority.toBytes()],
    programId
);

const buildMatchPDA = (
  mint: web3.PublicKey,
  userA: web3.PublicKey,
  programId: web3.PublicKey
) =>
  web3.PublicKey.findProgramAddressSync(
    [b`match`, userA.toBytes(), mint.toBytes()],
    programId
  );


const airdrop = async (
  pubKey: web3.PublicKey,
  amount: number,
  connection: web3.Connection
) => {
  const tx = await connection.requestAirdrop(
    pubKey,
    web3.LAMPORTS_PER_SOL * amount
  );
  const latestBlockHash = await connection.getLatestBlockhash();
  return connection.confirmTransaction({
    blockhash: latestBlockHash.blockhash,
    lastValidBlockHeight: latestBlockHash.lastValidBlockHeight,
    signature: tx,
  });
};

describe("madscape", () => {
  anchor.setProvider(anchor.AnchorProvider.env());
  const program = anchor.workspace.Madscape as Program<Madscape>;

  const initialFeeLamportsBasisPoints = 1000; // 10%
  const actualFeeLamportsBasisPoints = 500; // 5%
  const initSol = 5; // 5 SOL
  const escrowAmountSol = 1; // 1 SOL
  const fee = (escrowAmountSol * actualFeeLamportsBasisPoints) / 10_000;

  let mintOpen: web3.PublicKey = web3.Keypair.generate().publicKey;
  let mintPrivate: web3.PublicKey = web3.Keypair.generate().publicKey;;

  let mintOpenMint: web3.PublicKey = web3.Keypair.generate().publicKey;;
  let mintPrivateMint: web3.PublicKey = web3.Keypair.generate().publicKey;;

  let treasury: web3.Keypair = null;

  let userA: web3.Keypair = null;
  let userAMintXTA: web3.PublicKey = null;
  let userBMintXTA: web3.PublicKey = null;

  let userB: web3.Keypair = null;
  let userBMintYTA: web3.PublicKey = null;

  let mintX: web3.PublicKey = null;
  let mintXAuthority: web3.Keypair = null;
  let mintY: web3.PublicKey = null;
  let mintYAuthority: web3.Keypair = null;

  let approvedFeeMintAuthority: web3.Keypair = null;

  const wallet = (program.provider as anchor.AnchorProvider).wallet;

  before(async () => {
    treasury = web3.Keypair.generate();
    userA = web3.Keypair.generate();
    userB = web3.Keypair.generate();
    approvedFeeMintAuthority = web3.Keypair.generate();
    mintXAuthority = web3.Keypair.generate();
    mintYAuthority = web3.Keypair.generate();

    const approvedFeeMintAuthorityPK = approvedFeeMintAuthority.publicKey;
    const userBPK = userB.publicKey;
    const userAPK = userA.publicKey;
    const treasuryPK = treasury.publicKey;
    const mintXAuthorityPK = mintXAuthority.publicKey;
    const mintYAuthorityPK = mintYAuthority.publicKey;

    await Promise.all([
      airdrop(treasuryPK, initSol, program.provider.connection),
      airdrop(userAPK, initSol, program.provider.connection),
      airdrop(userBPK, initSol, program.provider.connection),
      airdrop(approvedFeeMintAuthorityPK, initSol, program.provider.connection),
      airdrop(mintXAuthorityPK, initSol, program.provider.connection),
      airdrop(mintYAuthorityPK, initSol, program.provider.connection),
    ]);

    mintX = await createMint(
      program.provider.connection,
      mintXAuthority,
      mintXAuthorityPK,
      null,
      0
    );
    userAMintXTA = await createAssociatedTokenAccount(
      program.provider.connection,
      userA,
      mintX,
      userAPK
    );
    userBMintXTA = await createAssociatedTokenAccount(
      program.provider.connection,
      userB,
      mintX,
      userBPK
    );
    mintY = await createMint(
      program.provider.connection,
      mintYAuthority,
      mintYAuthorityPK,
      null,
      0
    );
    userBMintYTA = await createAssociatedTokenAccount(
      program.provider.connection,
      userB,
      mintY,
      userBPK
    );
    await Promise.all([
      mintTo(
        program.provider.connection,
        mintXAuthority,
        mintX,
        userAMintXTA,
        mintXAuthority,
        10 * web3.LAMPORTS_PER_SOL
      ),
      mintTo(
        program.provider.connection,
        mintXAuthority,
        mintX,
        userBMintXTA,
        mintXAuthority,
        10 * web3.LAMPORTS_PER_SOL
      ),
      mintTo(
        program.provider.connection,
        mintYAuthority,
        mintY,
        userBMintYTA,
        mintYAuthority,
        1
      ),
    ]);
    //#endregion
  });

  it("Creates release authority", async () => {
    const authority = wallet.publicKey;
    const [releaseAuthority] = buildReleaseAuthorityPDA(
      authority,
      program.programId
    );
    console.log(program.programId.toString());
    const txId = await program.methods
      .createReleaseAuthority(initialFeeLamportsBasisPoints)
      .accountsStrict({
        authority,
        releaseAuthority,
        systemProgram: web3.SystemProgram.programId,
        treasury: treasury.publicKey,
      })
      .rpc();
    expect(txId).to.be.ok;
  });

  it("Updates release authority", async () => {
    const authority = wallet.publicKey;
    const [releaseAuthority] = buildReleaseAuthorityPDA(
      authority,
      program.programId
    );
    const txId = await program.methods
      .updateReleaseAuthority(actualFeeLamportsBasisPoints)
      .accountsStrict({
        authority,
        releaseAuthority,
        treasury: treasury.publicKey,
      })
      .rpc();
    expect(txId).to.be.ok;
  });

  it("Approves fee mint", async () => {
    const authority = wallet.publicKey;
    const [releaseAuthority] = buildReleaseAuthorityPDA(
      authority,
      program.programId
    );
    const txId = await program.methods
      // 1e8 = 0.1 for static fees on mints.
      .approveFeeMint(new anchor.BN(1e8))
      .accountsStrict({
        authority,
        releaseAuthority,
        mint: mintX,
      })
      .rpc();
    expect(txId).to.be.ok;
  });
  // it("Revokes fee mint", async () => {
  //   const authority = wallet.publicKey;
  //   const [releaseAuthority] = buildReleaseAuthorityPDA(
  //     authority,
  //     program.programId
  //   );
  //   const txId = await program.methods
  //     .revokeFeeMint()
  //     .accountsStrict({
  //       authority,
  //       releaseAuthority,
  //       mint: mintX,
  //     })
  //     .rpc();
  //   expect(txId).to.be.ok;
  // });
  it("Creates a public match", async () => {
    const authority = wallet.publicKey;
    const [releaseAuthority] = buildReleaseAuthorityPDA(
      authority,
      program.programId
    );
    const [game] = buildMatchPDA(
      mintOpen,
      userA.publicKey,
      program.programId
    );

    const tx = await program.methods
      .createOpenMatch(new anchor.BN(escrowAmountSol * web3.LAMPORTS_PER_SOL), 1)
      .accountsStrict({
        releaseAuthority,
        game,
        mint: mintOpen,
        userA: userA.publicKey,
        recentSlothashes: web3.SYSVAR_SLOT_HASHES_PUBKEY,
        systemProgram: web3.SystemProgram.programId,
      })
      .signers([userA])
      .transaction();

    const txId = await web3.sendAndConfirmTransaction(
      program.provider.connection,
      tx,
      [userA]
    );
    expect(txId).to.be.ok;
  });
  it("Creates a private match", async () => {
    const authority = wallet.publicKey;
    const [releaseAuthority] = buildReleaseAuthorityPDA(
      authority,
      program.programId
    );
    const [game] = buildMatchPDA(
      mintPrivate,
      userA.publicKey,
      program.programId
    );
    const tx = await program.methods
      .createPrivateMatch(new anchor.BN(escrowAmountSol * web3.LAMPORTS_PER_SOL), 1, userB.publicKey)
      .accountsStrict({
        releaseAuthority,
        game,
        mint: mintPrivate,
        userA: userA.publicKey,
        recentSlothashes: web3.SYSVAR_SLOT_HASHES_PUBKEY,
        systemProgram: web3.SystemProgram.programId,
      })
      .signers([userA])
      .transaction();

    const txId = await web3.sendAndConfirmTransaction(
      program.provider.connection,
      tx,
      [userA]
    );
    expect(txId).to.be.ok;
  });

  it("Joins a public match", async () => {
    const authority = wallet.publicKey;
    const [releaseAuthority] = buildReleaseAuthorityPDA(
      authority,
      program.programId
    );

    const [game] = buildMatchPDA(
      mintOpen,
      userA.publicKey,
      program.programId
    );
    const gamePreBalance = await program.provider.connection.getBalance(game);
    const tx = await program.methods
    .joinMatch()
    .accountsStrict({
      releaseAuthority,
      game,
      mint: mintOpen,
      userB: userB.publicKey,
      userA: userA.publicKey,
      systemProgram: web3.SystemProgram.programId,
    })
    .signers([userB])
    .transaction();

    const txId = await web3.sendAndConfirmTransaction(
      program.provider.connection,
      tx,
      [userB]
    );
    expect(txId).to.be.ok;

    const gameAccountBalance = await program.provider.connection.getBalance(game);
    const gameAccountAfter = await program.account.match.fetch(game);
    expect(gameAccountAfter.active).to.be.eq(true);
    expect(gameAccountBalance).to.be.eq(escrowAmountSol * web3.LAMPORTS_PER_SOL + gamePreBalance)
    expect(gameAccountAfter.userB.toString()).to.be.eq(userB.publicKey.toString());
  });

  it("End a match", async () => {
    const authority = wallet.publicKey;
    const [releaseAuthority] = buildReleaseAuthorityPDA(
      authority,
      program.programId
    );

    const [game] = buildMatchPDA(
      mintOpen,
      userA.publicKey,
      program.programId
    );
    const tx = await program.methods
    .endMatch(userB.publicKey)
    .accountsStrict({
      releaseAuthority,
      game,
      mint: mintOpen,
      userB: userB.publicKey,
      userA: userA.publicKey,
      treasury: treasury.publicKey,
      signer: authority,
    })
    .signers([(wallet as NodeWallet).payer])
    .transaction();

    const txId = await web3.sendAndConfirmTransaction(
      program.provider.connection,
      tx,
      [(wallet as NodeWallet).payer]
    );
    expect(txId).to.be.ok;
  });

  // it("Cancels a private match", async () => {
  //   const authority = wallet.publicKey;
  //   const [releaseAuthority] = buildReleaseAuthorityPDA(
  //     authority,
  //     program.programId
  //   );

  //   const [game] = buildMatchPDA(
  //     mintPrivate,
  //     userA.publicKey,
  //     program.programId
  //   );
  //   const tx = await program.methods
  //   .cancelPrivateMatch()
  //   .accountsStrict({
  //     releaseAuthority,
  //     game,
  //     mint: mintPrivate,
  //     userA: userA.publicKey,
  //     signer: authority,
  //   })
  //   .signers([(wallet as NodeWallet).payer])
  //   .transaction();

  //   const txId = await web3.sendAndConfirmTransaction(
  //     program.provider.connection,
  //     tx,
  //     [(wallet as NodeWallet).payer]
  //   );
  //   expect(txId).to.be.ok;
  // })

  // it("Cancels a public match", async () => {
  //   const authority = wallet.publicKey;
  //   const [releaseAuthority] = buildReleaseAuthorityPDA(
  //     authority,
  //     program.programId
  //   );
  //   const [game] = buildMatchPDA(
  //     mintOpen,
  //     userA.publicKey,
  //     program.programId
  //   );
  //   const tx = await program.methods
  //   .cancelOpenMatch()
  //   .accountsStrict({
  //     releaseAuthority,
  //     game,
  //     mint: mintOpen,
  //     userA: userA.publicKey,
  //   })
  //   .signers([userA])
  //   .transaction();

  //   const txId = await web3.sendAndConfirmTransaction(
  //     program.provider.connection,
  //     tx,
  //     [userA]
  //   );
  //   expect(txId).to.be.ok;
  // });

  // it("Joins a private match", async () => {
  //   const authority = wallet.publicKey;
  //   const [releaseAuthority] = buildReleaseAuthorityPDA(
  //     authority,
  //     program.programId
  //   );

  //   const [game] = buildMatchPDA(
  //     mintPrivate,
  //     userA.publicKey,
  //     program.programId
  //   );
  //   const gamePreBalance = await program.provider.connection.getBalance(game);
  //   const tx = await program.methods
  //   .joinMatch()
  //   .accountsStrict({
  //     releaseAuthority,
  //     game,
  //     mint: mintPrivate,
  //     userB: userB.publicKey,
  //     userA: userA.publicKey,
  //     systemProgram: web3.SystemProgram.programId,
  //   })
  //   .signers([userB])
  //   .transaction();

  //   const txId = await web3.sendAndConfirmTransaction(
  //     program.provider.connection,
  //     tx,
  //     [userB]
  //   );
  //   expect(txId).to.be.ok;

  //   const gameAccountBalance = await program.provider.connection.getBalance(game);
  //   const gameAccountAfter = await program.account.match.fetch(game);
  //   expect(gameAccountAfter.active).to.be.eq(true);
  //   expect(gameAccountBalance).to.be.eq(escrowAmountSol * web3.LAMPORTS_PER_SOL + gamePreBalance)
  //   expect(gameAccountAfter.userB.toString()).to.be.eq(userB.publicKey.toString());
  // });

  it("Creates open match with mint", async () => {
    const authority = wallet.publicKey;
    const [releaseAuthority] = buildReleaseAuthorityPDA(
      authority,
      program.programId
    );

    const [game] = buildMatchPDA(
      mintOpenMint,
      userA.publicKey,
      program.programId
    );

    const matchMintUserAAta = await getAssociatedTokenAddress(
      mintX,
      userA.publicKey,
      true
    )

    const matchMintMatchAta = await getAssociatedTokenAddress(
      mintX,
      game,
      true
    )

    const tx = await program.methods
    .createOpenMatchMint(new anchor.BN(escrowAmountSol * web3.LAMPORTS_PER_SOL), 1)
    .accountsStrict({
      releaseAuthority,
      game,
      mint: mintOpenMint,
      matchMint: mintX,
      userA: userA.publicKey,
      matchTokenAccount: matchMintMatchAta,
      userATokenAccount: matchMintUserAAta,
      recentSlothashes: web3.SYSVAR_SLOT_HASHES_PUBKEY,
      systemProgram: web3.SystemProgram.programId,
      tokenProgram: TOKEN_PROGRAM_ID,
      associatedTokenProgram: ASSOCIATED_TOKEN_PROGRAM_ID,
    })
    .signers([userA])
    .transaction();

    const txId = await web3.sendAndConfirmTransaction(
      program.provider.connection,
      tx,
      [userA]
    );
    expect(txId).to.be.ok;
  });

  it("Creates private match with mint", async () => {
    const authority = wallet.publicKey;
    const [releaseAuthority] = buildReleaseAuthorityPDA(
      authority,
      program.programId
    );

    const [game] = buildMatchPDA(
      mintPrivateMint,
      userA.publicKey,
      program.programId
    );

    const matchMintUserAAta = await getAssociatedTokenAddress(
      mintX,
      userA.publicKey,
      true
    )

    const matchMintMatchAta = await getAssociatedTokenAddress(
      mintX,
      game,
      true
    )

    const tx = await program.methods
    .createPrivateMatchMint(new anchor.BN(escrowAmountSol * web3.LAMPORTS_PER_SOL), 1, userB.publicKey)
    .accountsStrict({
      releaseAuthority,
      game,
      mint: mintPrivateMint,
      matchMint: mintX,
      userA: userA.publicKey,
      matchTokenAccount: matchMintMatchAta,
      userATokenAccount: matchMintUserAAta,
      recentSlothashes: web3.SYSVAR_SLOT_HASHES_PUBKEY,
      systemProgram: web3.SystemProgram.programId,
      tokenProgram: TOKEN_PROGRAM_ID,
      associatedTokenProgram: ASSOCIATED_TOKEN_PROGRAM_ID

    })
    .signers([userA])
    .transaction();

    const txId = await web3.sendAndConfirmTransaction(
      program.provider.connection,
      tx,
      [userA]
    );
    expect(txId).to.be.ok;    
  });
  it("Joins a public match with mint", async () => {
    const authority = wallet.publicKey;
    const [releaseAuthority] = buildReleaseAuthorityPDA(
      authority,
      program.programId
    );

    const [game] = buildMatchPDA(
      mintOpenMint,
      userA.publicKey,
      program.programId
    );

    const matchMintUserBAta = await getOrCreateAssociatedTokenAccount(
      program.provider.connection,
      userB,
      mintX,
      userB.publicKey,
      true
    )

    const matchMintMatchAta = await getOrCreateAssociatedTokenAccount(
      program.provider.connection,
      userB,
      mintX,
      game,
      true
    )

    const tx = await program.methods
    .joinMatchMint()
    .accountsStrict({
      releaseAuthority,
      game,
      mint: mintOpenMint,
      matchMint: mintX,
      userB: userB.publicKey,
      userA: userA.publicKey,
      matchTokenAccount: matchMintMatchAta.address,
      userBTokenAccount: matchMintUserBAta.address,
      systemProgram: web3.SystemProgram.programId,
      tokenProgram: TOKEN_PROGRAM_ID,
    })
    .signers([userB])
    .transaction();

    const txId = await web3.sendAndConfirmTransaction(
      program.provider.connection,
      tx,
      [userB]
    );
    expect(txId).to.be.ok;
  })
  it("End a match with mint", async () => {
    const authority = wallet.publicKey;
    const [releaseAuthority] = buildReleaseAuthorityPDA(
      authority,
      program.programId
    );

    const [game] = buildMatchPDA(
      mintOpenMint,
      userA.publicKey,
      program.programId
    );

    const treasuryFeeRecipientTokenAccount = await getOrCreateAssociatedTokenAccount(
      program.provider.connection,
      treasury,
      mintX,
      treasury.publicKey,
      true
    )

    const matchMintMatchAta = await getOrCreateAssociatedTokenAccount(
      program.provider.connection,
      userB,
      mintX,
      game,
      true
    )

    const tx = await program.methods
    .endMatchMint()
    .accountsStrict({
      releaseAuthority,
      game,
      mint: mintOpenMint,
      matchMint: mintX,
      userA: userA.publicKey,
      treasuryFeeRecipientTokenAccount: treasuryFeeRecipientTokenAccount.address,
      winner: userB.publicKey,
      winnerRecipientTokenAccount: userBMintXTA,
      treasury: treasury.publicKey,
      signer: authority,
      tokenProgram: TOKEN_PROGRAM_ID,
      systemProgram: web3.SystemProgram.programId,
      userB: userB.publicKey, // Add userB property
      matchTokenAccount: matchMintMatchAta.address, // Add matchTokenAccount property
    })
    .signers([(wallet as NodeWallet).payer])
    .transaction();

    const txId = await web3.sendAndConfirmTransaction(
      program.provider.connection,
      tx,
      [(wallet as NodeWallet).payer]
    );
    expect(txId).to.be.ok;

    const gameAccountAfterBalance = await program.provider.connection.getTokenAccountBalance(matchMintMatchAta.address);
    expect(gameAccountAfterBalance.value.uiAmount).to.be.eq(0);
  })
});
