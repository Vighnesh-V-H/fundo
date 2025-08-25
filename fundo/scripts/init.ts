import "dotenv/config";
import * as anchor from "@coral-xyz/anchor";
import { Fundo } from "../target/types/fundo";
import idl from "../target/idl/fundo.json";
import fs from "fs";
import os from "os";

const main = async (cluster: string) => {
  try {
    const rpcUrls = {
      devnet: "https://api.devnet.solana.com",
      testnet: "https://api.testnet.solana.com",
      mainnet: "https://api.mainnet-beta.solana.com",
      localhost: "http://127.0.0.1:8899",
    };

    const rpcUrl = rpcUrls[cluster as keyof typeof rpcUrls] || rpcUrls.devnet;
    const connection = new anchor.web3.Connection(rpcUrl, "confirmed");

    const keypairPath = `${os.homedir()}/.config/solana/id.json`;

    if (!fs.existsSync(keypairPath)) {
      throw new Error(`
❌ Keypair file not found at ${keypairPath}

To fix this, run one of these commands:
  solana-keygen new --outfile ~/.config/solana/id.json
  solana-keygen new  (creates default keypair)
            `);
    }

    const keypairData = JSON.parse(fs.readFileSync(keypairPath, "utf-8"));
    const wallet = anchor.web3.Keypair.fromSecretKey(
      new Uint8Array(keypairData)
    );
    console.log(wallet.publicKey.toBase58());

    const provider = new anchor.AnchorProvider(
      connection,
      new anchor.Wallet(wallet),
      { commitment: "confirmed" }
    );

    anchor.setProvider(provider);

    const program = new anchor.Program<Fundo>(idl, provider);

    const [programStatePda] = anchor.web3.PublicKey.findProgramAddressSync(
      [Buffer.from("program_state")],
      program.programId
    );

    try {
      const state = await program.account.programState.fetch(programStatePda);
      console.log(
        `Program already initialized, status: ${state.isInitialized}`
      );
    } catch (error) {
      const tx = await program.methods
        .initialize()
        .accountsPartial({
          programState: programStatePda,
          owner: provider.wallet.publicKey,
          systemProgram: anchor.web3.SystemProgram.programId,
        })
        .rpc();

      await connection.confirmTransaction(tx, "finalized");
      console.log("Program initialized successfully.", tx);
    }
  } catch (error) {
    console.error("❌ Error:", error instanceof Error ? error.message : error);
    process.exit(1);
  }
};
const cluster: string = process.env.NEXT_PUBLIC_RPC_URL || "localhost";
main(cluster).catch((error) => console.log(error));
