import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import { PublicKey, Connection } from '@solana/web3.js'
import { Sendsolfromapda } from "../target/types/sendsolfromapda";
const { SystemProgram } = anchor.web3;

describe("Send SOL from a pda", () => {
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);
  const program = anchor.workspace.Sendsolfromapda as Program<Sendsolfromapda>;
  const wallet = provider.wallet as anchor.Wallet;
  const connection = new Connection("https://api.devnet.solana.com");
    it('PDA created', async () => {

      const [userStatsPDA, bump] = await PublicKey.findProgramAddress(
        [
          anchor.utils.bytes.utf8.encode('send_one_sol'),
          wallet.publicKey.toBuffer(),
        ],
        program.programId
      )
      const tx = await program.methods.initializePda(bump.valueOf())
        .accounts({
          user: wallet.publicKey,
          pda: userStatsPDA,
          systemProgram: SystemProgram.programId
        }).rpc()
        await connection.requestAirdrop(userStatsPDA, 1000000000);
        console.log(tx)
        console.log(userStatsPDA.toBase58());
      }
    )
  }
)
