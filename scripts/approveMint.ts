import * as anchor from "@coral-xyz/anchor";
import { Program, web3 } from "@coral-xyz/anchor"
import { Madscape } from "../target/types/madscape";
import { buildReleaseAuthorityPDA } from "./common";


const authority = new web3.PublicKey("DJdhfVWECPiAK1DiSSQFu9QE5TQ9coMxhj4jm6z2Jcgw");
const mint = new web3.PublicKey("72pzypYmWeAbDXvmep8tBVeXpvcbrcvv6QKtS4T1btkF");


(async () => {
    anchor.setProvider(anchor.AnchorProvider.env());
    const program = anchor.workspace.Madscape as Program<Madscape>;

    const [releaseAuthority, nonce] = await buildReleaseAuthorityPDA(authority, program.programId);

    const txId = await program.methods
        .approveFeeMint(new anchor.BN(1e8))
        .accountsStrict({
        authority,
        releaseAuthority,
        mint,
        })
      .rpc();

    console.log("txId", txId);

})()
