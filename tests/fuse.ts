import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Fuse } from "../target/types/fuse";
import { PublicKey, SystemProgram } from "@solana/web3.js";
import { token } from "@coral-xyz/anchor/dist/cjs/utils";

describe("fuse", () => {
  // Configure the client to use the local cluster.
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const program = anchor.workspace.Fuse as Program<Fuse>;

  const METADATA_SEED = 'metadata';
  const TOKEN_METADATA_PROGRAM_ID = new PublicKey('metaqbxxUerdq28cj1RbAWkYQm3ybzjb6a8bt518x1s');

  const MINT_SEED = 'fuse';
  const TOKEN_DETAILS_SEED = 'token'

  const payer = provider.wallet.publicKey;
  const metadata = {
    name: 'Fuse',
    symbol: 'FUSE',
    uri: 'https://fuse.com',
    decimals: 6,
  }
  const mintAmount = 1000;

  const [tokenDetails] = anchor.web3.PublicKey.findProgramAddressSync(
    [Buffer.from(TOKEN_DETAILS_SEED), payer.toBuffer()],
    program.programId
  );

  const [mint] = anchor.web3.PublicKey.findProgramAddressSync(
    [Buffer.from(MINT_SEED)],
    program.programId
  );

  const [metadataAddress] = anchor.web3.PublicKey.findProgramAddressSync(
    [Buffer.from(METADATA_SEED),
    TOKEN_METADATA_PROGRAM_ID.toBuffer(),
    mint.toBuffer()],
    TOKEN_METADATA_PROGRAM_ID
  );

  // it("Is initialized!", async () => {
  //   // const info = await program.account.tokenDetails.fetch(mint);

  //   // if (info) {
  //   //   return; 
  //   // }
  //   console.log("  Mint not found. Attempting to initialize.");

  //   const context = {
  //     metadata: metadataAddress,
  //     mint,
  //     payer,
  //     rent: anchor.web3.SYSVAR_RENT_PUBKEY,
  //     systemProgram: SystemProgram.programId,
  //     tokenProgram: anchor.utils.token.TOKEN_PROGRAM_ID,
  //     tokenMetadataProgram: TOKEN_METADATA_PROGRAM_ID,
  //     tokenDetails
  //   }
  //   console.log(context)

  //   const tx = await program.methods
  //     .createMint(metadata)
  //     .accountsStrict(context)
  //     .signers([]).rpc();

  //   console.log("  Mint initialized.", tx);
  // });

  it("mint tokens", async () => {

    const destination = await anchor.utils.token.associatedAddress({
      mint: mint,
      owner: payer,
    });

    let initialBalance: number;
    try {
      const balance = (await provider.connection.getTokenAccountBalance(destination))
      initialBalance = balance.value.uiAmount;
    } catch {
      // Token account not yet initiated has 0 balance
      initialBalance = 0;
    } 
    
    const context = {
      mint,
      destination,
      payer,
      rent: anchor.web3.SYSVAR_RENT_PUBKEY,
      systemProgram: anchor.web3.SystemProgram.programId,
      tokenProgram: anchor.utils.token.TOKEN_PROGRAM_ID,
      associatedTokenProgram: anchor.utils.token.ASSOCIATED_PROGRAM_ID,
    };

    const tx = await program.methods
      .mintTokens(new anchor.BN(mintAmount * 10 ** metadata.decimals))
      .accounts(context)
      .signers([]).rpc();

      console.log("you are the winner", tx)
    
    
  });

});
