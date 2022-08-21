import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import { Sendsolfromapda } from "../target/types/sendsolfromapda";

describe("Send SOL from a pda", () => {
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);
  const program = anchor.workspace.Sendsolfromapda as Program<Sendsolfromapda>;
  const wallet = provider.wallet as anchor.Wallet;
    it('SOL sent successfully', async () => {
        const to = new anchor.web3.PublicKey("AbQWyJxGzmxC51t4EjYCg4b3rhS5sCUB4BHKMZWLcKdZ");
        const pda = new anchor.web3.PublicKey("HXJxnyyCV3QvD2sokV4FSQHEmfpgtPwuv1gLYuegNGeE");
        const tx = await program.methods.sendOneSolFromAPda(new anchor.BN(999046480))
        .accounts({
          pda: pda,
          to: to,
          user: wallet.publicKey
        })
        .rpc()
        console.log(tx)
      }
    )
  }
)