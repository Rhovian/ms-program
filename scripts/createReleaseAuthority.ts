import * as anchor from "@coral-xyz/anchor";
import { Program, web3 } from "@coral-xyz/anchor"
import { Madscape } from "../target/types/madscape";
import { buildReleaseAuthorityPDA } from "./common";


const authority = new web3.PublicKey("A4c5nctuNSN7jTsjDahv6bAWthmUzmXi3yBocvLYM4Bz");
const initialFeeLamportsBasisPoints = 0; // 5%


(async () => {
    anchor.setProvider(anchor.AnchorProvider.env());
    const program = anchor.workspace.Madscape as Program<Madscape>;

    console.log(program.programId.toString());

    const [releaseAuthority, nonce] = await buildReleaseAuthorityPDA(authority, program.programId);

    const txId = await program.methods
      .createReleaseAuthority(initialFeeLamportsBasisPoints)
      .accountsStrict({
        authority,
        releaseAuthority,
        systemProgram: web3.SystemProgram.programId,
        treasury: authority,
      })
      .rpc();

    console.log("txId", txId);

})()
