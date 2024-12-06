import * as anchor from "@coral-xyz/anchor";
import { Program, web3 } from "@coral-xyz/anchor"
import { Madscape } from "../target/types/madscape";
import { buildReleaseAuthorityPDA } from "./common";


const releaseAuthority = new web3.PublicKey("FiFaaxfFc6F4N2qb6HmbFXbmNvErLiKsfDog9cJZGa9s");


(async () => {
    anchor.setProvider(anchor.AnchorProvider.env());
    const program = anchor.workspace.Madscape as Program<Madscape>;

    const accountInfo = await program.account.releaseAuthority.fetch(releaseAuthority);

    console.log("releaseAuthority", accountInfo);

})()
