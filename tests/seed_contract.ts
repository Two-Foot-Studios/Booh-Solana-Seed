import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { SeedContract } from "../target/types/seed_contract";
import {createMint, getOrCreateAssociatedTokenAccount, mintToChecked} from "@solana/spl-token";


const LAMPORT_PER_SOL = 1000000000;
const ADMIN_SECRET = new Uint8Array([130,  48, 202,   8,  31,  74, 143, 100,  64, 114, 166, 66, 132, 155,  54, 209, 118,  90,  56, 189, 216, 176, 168, 121, 232, 114,  31,  61, 196, 124, 175, 202, 143, 250,  34,  79, 254, 211, 237, 128,  37, 151, 253, 122, 189,  75, 128,  67, 236, 175,  68, 138, 101,  19,  21, 200, 254,  44, 208,  24,  65,  70,  98, 164]);
const TOKEN_SECRET = new Uint8Array([ 78, 234, 114,  83, 170,   3,  69, 179,  91,  13, 155, 96,  13,  61,  85, 197, 108, 208,  46,   4, 153,  11, 143,  40,  85, 215, 209, 162, 139, 218,  43,  18, 157, 189, 122, 245, 240, 128, 181, 240,  92, 228, 148, 144, 243,  95,  42, 165,  62,  51, 133,  86, 176,  57,  53, 44,  39, 130, 123,  73, 170, 244, 108, 235]);
const FROM_SECRET = new Uint8Array([ 96,  19, 253, 166,  70,  56,  89, 254, 232,  67, 247, 51, 238, 153, 180,  56, 212, 183, 162, 171, 132, 229, 78,  29, 238, 244, 190,  12, 164, 137, 107, 195,  73, 13, 206,  47, 230,  50,   0, 228, 245,  64, 226,  88, 56, 124, 114, 194, 169,  76,  36, 204, 108,   0,  77, 100, 166, 218,  20, 214,  48, 249, 241,  25]);
const BANK_TOKEN_SECRET = new Uint8Array([60, 114,  76, 167,  42, 113, 187, 227, 155,  71,  55, 221,   5, 182, 206,  70, 123, 225,  39, 115,  70, 240, 199, 193, 237, 218, 107, 245, 128, 191, 237, 215, 107, 6, 219, 180,  81, 190, 103, 139, 137,  18, 135, 149, 18, 250, 182,  36, 161, 151, 135,  88, 221,  73,  81, 112,  75, 181, 133,  65, 238,  68, 244, 202]);

describe("seed_contract", () => {
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const program = anchor.workspace.SeedContract as Program<SeedContract>;

  let mintPubKey = null;
  let bankPubKey = null;

  let adminAta = null;
  let fromAta = null;

  const adminWallet = anchor.web3.Keypair.fromSecretKey(ADMIN_SECRET);
  const fromWallet = anchor.web3.Keypair.fromSecretKey(FROM_SECRET);

  const authority = anchor.web3.Keypair.generate();

  const [mintBankPda, _mbp] = anchor.web3.PublicKey.findProgramAddressSync(
      [Buffer.from("mint_bank")],
      program.programId
  );

  const [mintStatPda, _msp] = anchor.web3.PublicKey.findProgramAddressSync(
      [Buffer.from("mint_stat")],
      program.programId
  );


  it("Is init!", async () => {
    console.log("Requesting airdrop..");

    await provider.connection.confirmTransaction(
        await provider.connection.requestAirdrop(authority.publicKey, LAMPORT_PER_SOL * 10),
        "confirmed"
    );

    await provider.connection.confirmTransaction(
        await provider.connection.requestAirdrop(adminWallet.publicKey, LAMPORT_PER_SOL * 10),
        "confirmed"
    );

    await provider.connection.confirmTransaction(
        await provider.connection.requestAirdrop(fromWallet.publicKey, LAMPORT_PER_SOL * 10),
        "confirmed"
    );

    console.log("Creating token mint for our token..");
    const mintKeypair = anchor.web3.Keypair.fromSecretKey(TOKEN_SECRET);
    mintPubKey = await createMint(
        provider.connection,
        authority,
        authority.publicKey,
        null,
        9,
        mintKeypair
    );

    console.log("Creating bank token mint..");
    const bankKeypair = anchor.web3.Keypair.fromSecretKey(BANK_TOKEN_SECRET);
    bankPubKey = await createMint(
        provider.connection,
        authority,
        authority.publicKey,
        null,
        9,
        bankKeypair
    );

    console.log("Creating ATA to 'Admin' wallet");
    adminAta = await getOrCreateAssociatedTokenAccount(
        provider.connection,
        adminWallet,
        mintPubKey,
        adminWallet.publicKey
    );

    console.log("Mint tokens to admin wallet");
    await mintToChecked(
        provider.connection,
        adminWallet,
        mintPubKey,
        adminAta.address,
        authority,
        8_000_000e18,
        9
    );
  });

  it("Init seed", async () => {
    let amount = new anchor.BN('72000000000000000');
    let beforeBalance = await provider.connection.getTokenAccountBalance(adminAta.address);
    console.log("Balance in our token before - ", beforeBalance.value.uiAmount);

    console.log("Init contract");

    await program.methods
        .initialize(amount)
        .accounts({
          mintOfToken: mintPubKey,
          admin: adminWallet.publicKey,
          adminTokenAta: adminAta.address,
          mintStat: mintStatPda,
          mintBank: mintBankPda
        })
        .signers([adminWallet])
        .rpc();

    let afterBalance = await provider.connection.getTokenAccountBalance(adminAta.address);
    console.log("Balance in our token after - ", afterBalance.value.uiAmount);

    let pdaBalance = await provider.connection.getTokenAccountBalance(mintBankPda);
    console.log("Pda seed balance - ", pdaBalance.value.uiAmount);

    let info = await program.account.mintStat.fetch(mintStatPda);
    console.log(
        "Mint stat: amount: ", info.amount.toString(),
        "\nAmount per account: ", info.amountPerAccount.toString(),
        "\nStart date: ", info.start.toString(),
        "\nEnd date: ", info.end.toString()
    );
  });

  it("Mint tokens", async () => {
    fromAta = await getOrCreateAssociatedTokenAccount(provider.connection, fromWallet, mintPubKey, fromWallet.publicKey);

    let beforeBalance = await provider.connection.getTokenAccountBalance(fromAta.address);
    console.log("Balance in our token before - ", beforeBalance.value.uiAmount);

    const [fromStatPda, _fsp] = anchor.web3.PublicKey.findProgramAddressSync(
        [fromWallet.publicKey.toBuffer()],
        program.programId
    );

    await program.methods
        .mint()
        .accounts({
          mintOfToken: mintPubKey,
          userTokenAta: fromAta.address,
          user: fromWallet.publicKey,
          mintStat: mintStatPda,
          mintBank: mintBankPda,
          userMintStat: fromStatPda
        })
        .signers([fromWallet])
        .preInstructions([
            anchor.web3.ComputeBudgetProgram.setComputeUnitLimit({
              units: 400_000
            })
        ])
        .rpc();

    let afterBalance = await provider.connection.getTokenAccountBalance(fromAta.address);
    console.log("After balance in our token - ", afterBalance.value.uiAmount);

    let info = await program.account.userMintStat.fetch(fromStatPda);
    console.log("\nLast reward: ", info.lastReward.toString());
  })
});
