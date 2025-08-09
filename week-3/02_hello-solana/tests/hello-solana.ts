import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { HelloSolana } from "../target/types/hello_solana";

describe("hello-solana", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.helloSolana as Program<HelloSolana>;
  
  const signer = anchor.web3.Keypair.generate();
  const data_account = anchor.web3.Keypair.generate();


  it("Is initialized!", async () => {
    await program.provider.connection.confirmTransaction(
      await program.provider.connection.requestAirdrop(signer.publicKey, 100*anchor.web3.LAMPORTS_PER_SOL),
      "confirmed"
    );
    // Add your test here.
    

    const tx = await program.methods
      .initialize("Hello World")
      .accounts({
        // system_program <- setted automatically by anchor
        signer: signer.publicKey,
        dataAccount: data_account.publicKey,
      })
      .signers([signer, data_account])
      .rpc();

    const dataAccount = await program.account.whatever.fetch(data_account.publicKey);

    console.log("Signature: ", tx);
    console.log("Data Account: ", dataAccount);
  });
});
