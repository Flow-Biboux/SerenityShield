import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import { Presale } from "../target/types/presale";

describe("presale", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.Provider.env());

  const program = anchor.workspace.Presale as Program<Presale>;

  it("Create_Presale!", async () => {
    // const tx = await program.rpc.createSERSHPresale({});
  });
  it("Normal Buy!", async () => {
    // const tx = await program.rpc.buySERSH({});
  });
  it("Referal buy!", async () => {
    // const tx = await program.rpc.buySERSH({});
  });
  it("Claim Vested!", async () => {
    // const tx = await program.rpc.claimVestedSERSH({});
  });
});
