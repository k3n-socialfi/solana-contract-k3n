import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { K3n } from "../target/types/k3n";

describe("k3n", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.K3n as Program<K3n>;

  it("Is initialized!", async () => {
    // Add your test here.
    const tx = await program.methods.initialize().rpc();
    console.log("Your transaction signature", tx);
  });
});
