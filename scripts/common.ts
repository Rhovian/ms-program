import * as anchor from "@coral-xyz/anchor";
import { Program, web3 } from "@coral-xyz/anchor";
import { PublicKey } from "@solana/web3.js";
import { Madscape } from "../target/types/madscape";
import { TextEncoder } from "util";

anchor.setProvider(anchor.AnchorProvider.env());
export const program = anchor.workspace.Madscape as Program<Madscape>;

const encode = (str: string) => new TextEncoder().encode(str);
const b = (input: TemplateStringsArray) => encode(input.join(""));

export const buildReleaseAuthorityPDA = (
  authority: web3.PublicKey,
  programId: web3.PublicKey
) =>
  web3.PublicKey.findProgramAddressSync(
    [b`release`, authority.toBytes()],
    programId
);
