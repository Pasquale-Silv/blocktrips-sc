import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { BlocktripsSc } from "../target/types/blocktrips_sc";
import { PublicKey } from "@solana/web3.js";

describe("blocktrips-sc", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.BlocktripsSc as Program<BlocktripsSc>;

  it("Is initialized!", async () => {
    // Add your test here.
    const accommodation_business = new PublicKey("5Y88bWffMahHof1unqAc98gf3T1cokUAuEGFtCaetG3v");
    const date_of_departure = "20240811";
    const end_date = "20240820";
    const price = 4;

    const tx = await program.methods.initialize(accommodation_business, date_of_departure, end_date, price).rpc();
    console.log("Your transaction signature", tx);
  });
});
